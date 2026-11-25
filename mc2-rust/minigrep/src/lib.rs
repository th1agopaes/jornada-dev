//! Biblioteca do `minigrep`, um `grep` pequeno: procura um texto nas linhas de um arquivo.
//!
//! Baseado no projeto do Rust Book ([cap. 12] e [13.3]). Tudo o que dá para testar mora aqui;
//! o executável (`src/main.rs`) só lê o ambiente, chama [`executar`] e trata os erros.
//!
//! [cap. 12]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html
//! [13.3]: https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html

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

#[cfg(test)]
mod tests {
    use super::*;

    const TEXTO: &str = "\
Slime: fraco contra fogo.
Mímico: finge ser um baú.
Fênix: renasce do Fogo.
Esqueleto: anda com o MÍMICO.";

    fn numeros(consulta: &str, caixa: Caixa) -> Vec<usize> {
        buscar(consulta, TEXTO, caixa)
            .map(|ocorrencia| ocorrencia.numero)
            .collect()
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
}
