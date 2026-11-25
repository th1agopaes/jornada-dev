//! Biblioteca do `minigrep`, um `grep` pequeno: procura um texto nas linhas de um arquivo.
//!
//! Baseado no projeto do Rust Book ([cap. 12] e [13.3]). Tudo o que dá para testar mora aqui;
//! o executável (`src/main.rs`) só lê o ambiente, chama [`executar`] e trata os erros.
//!
//! [cap. 12]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html
//! [13.3]: https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html

use std::error::Error;
use std::fmt;

use std::fs;
use std::io::Write;

/// Como a busca trata maiúsculas e minúsculas.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Caixa {
    /// `"fogo"` não casa com `"Fogo"` (o padrão, como no `grep`)
    Diferenciar,
    /// `"fogo"` casa com `"Fogo"` e `"FOGO"`, inclusive com acento (`"mímico"` e `"MÍMICO"`).
    Ignorar,
}

/// Uma linha que casou com a consulta.
#[derive(Debug, PartialEq, Eq)]
pub struct Ocorrencia<'a> {
    /// Número da linha no texto, contando a partir de 1 (como no `grep -n`).
    pub numero: usize,
    /// A linha inteira, sem o fim de linha: uma fatia do texto original, sem cópia.
    pub linha: &'a str,
}

/// Procura `consulta` em cada linha de `texto` e devolve as ocorrências, na ordem do texto.
///
/// A esteira é preguiçosa: cada linha só é testada quando alguém pede a próxima ocorrência.
/// As ocorrências emprestam de `texto` (o `'a`), então valem enquanto ele valer. Aceita fim de
/// linha `\n` e `\r\n`.
///
/// # Examples
///
/// ```
/// use minigrep::{Caixa, buscar};
///
/// let texto = "Slime: fraco contra fogo.\nFênix: renasce do Fogo.";
///
/// let numeros: Vec<usize> = buscar("fogo", texto, Caixa::Diferenciar)
///     .map(|ocorrencia| ocorrencia.numero)
///     .collect();
/// assert_eq!(numeros, [1]);
///
/// let linhas: Vec<&str> = buscar("FOGO", texto, Caixa::Ignorar)
///     .map(|ocorrencia| ocorrencia.linha)
///     .collect();
/// assert_eq!(linhas, ["Slime: fraco contra fogo.", "Fênix: renasce do Fogo."]);
/// ```
pub fn buscar<'a>(
    consulta: &str,
    texto: &'a str,
    caixa: Caixa,
) -> impl Iterator<Item = Ocorrencia<'a>> {
    // Uma vez só, fora da esteira; `to_lowercase` entende acentos (`Í` vira `í`)
    let consulta_min = consulta.to_lowercase();

    texto
        .lines()
        .enumerate()
        .filter(move |(_, linha)| match caixa {
            Caixa::Diferenciar => linha.contains(consulta),
            Caixa::Ignorar => linha.to_lowercase().contains(&consulta_min),
        })
        .map(|(indice, linha)| Ocorrencia {
            numero: indice + 1,
            linha,
        })
}

/// O que o `minigrep` vai fazer, montado a partir da linha de comando.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// O texto procurado em cada linha.
    pub consulta: String,
    /// O caminho do arquivo onde procurar.
    pub arquivo: String,
    /// Como tratar maiúsculas e minúsculas.
    pub caixa: Caixa,
    /// Se cada linha sai com o número dela na frente (`-n`).
    pub numerar: bool,
}

impl Config {
    /// Monta a configuração a partir dos argumentos do programa.
    ///
    /// `args` vem como o sistema entrega: o primeiro é o nome do programa, e é pulado.
    /// `ignorar_caixa` diz se a variável de ambiente `IGNORE_CASE` está definida; quem chama lê
    /// o ambiente e passa a resposta, então os testes não precisam mexer nele.
    ///
    /// Opções, em qualquer posição: `-i` ignora maiúsculas, `-s` diferencia e `-n` numera as
    /// linhas. Uma opção sempre ganha da variável de ambiente; entre `-i` e `-s`, vale a última.
    ///
    /// # Errors
    ///
    /// Devolve um [`ErroArgs`] se faltar a consulta ou o arquivo, se sobrar argumento ou se
    /// aparecer uma opção que não existe.
    ///
    /// # Examples
    ///
    /// ```
    /// use minigrep::{Caixa, Config};
    ///
    /// let args = ["minigrep", "-n", "fogo", "bestiario.txt"].map(String::from);
    /// let config = Config::build(args.into_iter(), true)?;
    ///
    /// assert_eq!(config.consulta, "fogo");
    /// assert_eq!(config.caixa, Caixa::Ignorar); // veio do `IGNORE_CASE`
    /// assert!(config.numerar);
    /// # Ok::<(), minigrep::ErroArgs>(())
    /// ```
    pub fn build(
        mut args: impl Iterator<Item = String>,
        ignorar_caixa: bool,
    ) -> Result<Self, ErroArgs> {
        args.next(); // o nome do programa

        let mut caixa = if ignorar_caixa {
            Caixa::Ignorar
        } else {
            Caixa::Diferenciar
        };
        let mut numerar = false;
        let mut posicionais = Vec::new();

        for arg in args {
            match arg.as_str() {
                "-i" => caixa = Caixa::Ignorar,
                "-s" => caixa = Caixa::Diferenciar,
                "-n" => numerar = true,
                _ if arg.starts_with('-') => return Err(ErroArgs::OpcaoDesconhecida(arg)),
                _ => posicionais.push(arg),
            }
        }

        let mut posicionais = posicionais.into_iter();
        let consulta = posicionais.next().ok_or(ErroArgs::FaltaConsulta)?;
        let arquivo = posicionais.next().ok_or(ErroArgs::FaltaArquivo)?;
        if let Some(sobra) = posicionais.next() {
            return Err(ErroArgs::ArgumentoSobrando(sobra));
        }

        Ok(Self {
            consulta,
            arquivo,
            caixa,
            numerar,
        })
    }
}

/// Erro na linha de comando.
#[derive(Debug, PartialEq, Eq)]
pub enum ErroArgs {
    /// Não veio nenhum argumento além das opções.
    FaltaConsulta,
    /// Veio a consulta, mas não o arquivo.
    FaltaArquivo,
    /// Opção que o `minigrep` não conhece; guarda o texto lido (`-x`).
    OpcaoDesconhecida(String),
    /// Argumento a mais depois do arquivo.
    ArgumentoSobrando(String),
}

impl fmt::Display for ErroArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FaltaConsulta => write!(f, "faltou a consulta"),
            Self::FaltaArquivo => write!(f, "faltou o arquivo"),
            Self::OpcaoDesconhecida(opcao) => write!(f, "opção desconhecida: {opcao}"),
            Self::ArgumentoSobrando(arg) => write!(f, "argumento a mais: {arg}"),
        }
    }
}

impl Error for ErroArgs {}

/// Lê o arquivo da configuração e escreve em `saida` as linhas que casaram (o `run` do livro).
///
/// Devolve quantas linhas casaram; o executável usa esse número no código de saída.
/// `saida` é a tela no executável e um `Vec<u8>` nos testes.
///
/// # Errors
///
/// Falha se o arquivo não puder ser lido (não existe, sem permissão ou não é UTF-8) ou se a
/// escrita em `saida` falhar.
pub fn executar(config: &Config, saida: &mut impl Write) -> Result<usize, Box<dyn Error>> {
    // O `io::Error` sozinho não diz qual arquivo falhou; o `?` converte a `String` pelo `From`
    let texto = fs::read_to_string(&config.arquivo)
        .map_err(|erro| format!("{}: {erro}", config.arquivo))?;

    let mut total = 0;
    for Ocorrencia { numero, linha } in buscar(&config.consulta, &texto, config.caixa) {
        if config.numerar {
            writeln!(saida, "{numero}:{linha}")?;
        } else {
            writeln!(saida, "{linha}")?;
        }
        total += 1;
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTO: &str = "\
Slime: fraco contra fogo.
Mímico: finge ser um baú.
Fênix: renasce do Fogo.
Esqueleto: anda com o MÍMICO.";

    // O exemplo do repositório também é a fixture dos testes de `executar`
    const BESTIARIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/exemplos/bestiario.txt");

    fn numeros(consulta: &str, caixa: Caixa) -> Vec<usize> {
        buscar(consulta, TEXTO, caixa)
            .map(|ocorrencia| ocorrencia.numero)
            .collect()
    }

    fn config(args: &[&str]) -> Result<Config, ErroArgs> {
        let args = ["minigrep"]
            .iter()
            .chain(args)
            .map(|arg| (*arg).to_string());
        Config::build(args, false)
    }

    // --- buscar ---

    #[test]
    fn diferencia_maiuscula_por_padrao() {
        assert_eq!(numeros("fogo", Caixa::Diferenciar), [1]);
        assert_eq!(numeros("Fogo", Caixa::Diferenciar), [3]);
    }

    #[test]
    fn ignora_maiusculas_inclusive_com_acento() {
        assert_eq!(numeros("fogo", Caixa::Ignorar), [1, 3]);
        assert_eq!(numeros("mímico", Caixa::Ignorar), [2, 4]);
    }

    #[test]
    fn ocorrencia_traz_o_numero_e_a_linha_inteira() {
        let esperado = Ocorrencia {
            numero: 2,
            linha: "Mímico: finge ser um baú.",
        };
        let primeira = buscar("baú", TEXTO, Caixa::Diferenciar).next();
        assert_eq!(primeira, Some(esperado));
    }

    #[test]
    fn sem_ocorrencia_a_esteira_vem_vazia() {
        assert_eq!(numeros("lich", Caixa::Ignorar), []);
    }

    #[test]
    fn consulta_vazia_casa_com_todas_as_linhas() {
        assert_eq!(numeros("", Caixa::Diferenciar), [1, 2, 3, 4]);
    }

    #[test]
    fn aceita_fim_de_linha_do_windows() {
        let linhas: Vec<&str> = buscar("Golem", "Slime\r\nGolem\r\nLich", Caixa::Diferenciar)
            .map(|ocorrencia| ocorrencia.linha)
            .collect();
        assert_eq!(linhas, ["Golem"]);
    }

    // --- Config::build---

    #[test]
    fn le_consulta_e_arquivo_com_os_padroes() {
        let esperado = Config {
            consulta: "fogo".to_string(),
            arquivo: "bestiario.txt".to_string(),
            caixa: Caixa::Diferenciar,
            numerar: false,
        };
        assert_eq!(config(&["fogo", "bestiario.txt"]), Ok(esperado));
    }

    #[test]
    fn opcoes_valem_em_qualquer_posicao() {
        let config = config(&["fogo", "-n", "bestiario.txt", "-i"]).expect("argumentos válidos");
        assert_eq!((config.caixa, config.numerar), (Caixa::Ignorar, true));
    }

    #[test]
    fn opcao_ganha_da_variavel_de_ambiente() {
        let args = ["minigrep", "-s", "fogo", "bestiario.txt"].map(String::from);
        let config = Config::build(args.into_iter(), true).expect("argumentos válidos");
        assert_eq!(config.caixa, Caixa::Diferenciar);
    }

    #[test]
    fn entre_i_e_s_vale_a_ultima() {
        let config = config(&["-i", "-s", "fogo", "bestiario.txt"]).expect("argumentos válidos");
        assert_eq!(config.caixa, Caixa::Diferenciar);
    }

    #[test]
    fn recusa_argumentos_faltando_ou_sobrando() {
        assert_eq!(config(&[]), Err(ErroArgs::FaltaConsulta));
        assert_eq!(config(&["-n"]), Err(ErroArgs::FaltaConsulta));
        assert_eq!(config(&["fogo"]), Err(ErroArgs::FaltaArquivo));
        let sobra = config(&["fogo", "a.txt", "b.txt"]);
        assert_eq!(sobra, Err(ErroArgs::ArgumentoSobrando("b.txt".to_string())));
    }

    #[test]
    fn recusa_opcao_desconhecida() {
        let erro = config(&["-x", "fogo", "bestiario.txt"]);
        assert_eq!(erro, Err(ErroArgs::OpcaoDesconhecida("-x".to_string())));
    }

    // --- executar ---

    #[test]
    fn executar_escreve_as_linhas_e_conta() {
        let config = Config {
            consulta: "fogo".to_string(),
            arquivo: BESTIARIO.to_string(),
            caixa: Caixa::Ignorar,
            numerar: true,
        };
        let mut saida = Vec::new();

        let total = executar(&config, &mut saida).expect("o bestiário existe");

        assert_eq!(total, 3);
        let esperado = "\
3:Slime: fraco contra fogo; aparece em bandos perto da água.
5:Dragão Vermelho: imune a fogo; foge quando perde metade da vida.
7:Fênix: renasce do Fogo e só cai com gelo.
";
        assert_eq!(String::from_utf8(saida).expect("saída em UTF-8"), esperado);
    }

    #[test]
    fn executar_diz_qual_arquivo_falhou() {
        let config = Config {
            consulta: "fogo".to_string(),
            arquivo: "nao-existe.txt".to_string(),
            caixa: Caixa::Diferenciar,
            numerar: false,
        };

        let erro = executar(&config, &mut Vec::new()).expect_err("o arquivo não existe");

        // Depois do nome, a mensagem é do sistema operacional e muda entre Linux e Windows
        let mensagem = erro.to_string();
        assert!(mensagem.starts_with("nao-existe.txt: "), "veio: {mensagem}");
    }
}
