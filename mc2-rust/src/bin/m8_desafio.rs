//! MC2 · Rust do zero — Módulo 8 · Exercício: o mural de missões da guilda
//! Objetivo: closures, um iterador próprio, adaptadores e `collect` num mural de missões.
//! Rodar:    `cargo run --bin m8_desafio`

use std::cmp::Reverse;
use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

// --- 1. A missão ---------------------------------

// A ordem das variantes vira a ordem do `>=`: C < B < A < S
#[derive(Debug, PartialEq, PartialOrd)]
enum Rank {
    C,
    B,
    A,
    S,
}

// `Debug` + `PartialEq`: pedidos pelos testes, que comparam e imprimem missões (seção 7)
#[derive(Debug, PartialEq)]
struct Missao {
    nome: String,
    rank: Rank,
    recompensa: u32,
    local: String,
}

// "Selar o Mímico [B, 120 de ouro, Masmorra]"; o `{:?}` do Rank já é a letra
impl fmt::Display for Missao {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Desmonta a struct num `let` (o padrão `Variante { a, b }` do Módulo 4)
        let Missao {
            nome,
            rank,
            recompensa,
            local,
        } = self;
        write!(f, "{nome} [{rank:?}, {recompensa} de ouro, {local}]")
    }
}

// --- 2. Lendo o mural ---------------------------------

#[derive(Debug, PartialEq)] // o mesmo motivo: o teste compara o `Err` inteiro
enum ErroMural {
    Incompleta(String),        // faltou campo na linha (guarda a linha)
    Rank(String),              // rank fora da lista (guarda o texto)
    Recompensa(ParseIntError), // recompensa que não é número
}

impl fmt::Display for ErroMural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroMural::Incompleta(linha) => write!(f, "linha incompleta: {linha}"),
            ErroMural::Rank(texto) => write!(f, "rank desconhecido: {texto}"),
            ErroMural::Recompensa(erro) => write!(f, "recompensa inválida ({erro})"),
        }
    }
}

impl From<ParseIntError> for ErroMural {
    fn from(erro: ParseIntError) -> Self {
        ErroMural::Recompensa(erro)
    }
}

fn ler_rank(texto: &str) -> Option<Rank> {
    match texto {
        "C" => Some(Rank::C),
        "B" => Some(Rank::B),
        "A" => Some(Rank::A),
        "S" => Some(Rank::S),
        _ => None,
    }
}

// "Selar o Mímico | B | 120 | Masmorra" -> Missao
fn ler_missao(linha: &str) -> Result<Missao, ErroMural> {
    // `str::trim` no lugar de `|p| p.trim()`: uma função com nome também serve de closure
    let mut partes = linha.split('|').map(str::trim);
    let (Some(nome), Some(rank), Some(recompensa), Some(local)) =
        (partes.next(), partes.next(), partes.next(), partes.next())
    else {
        return Err(ErroMural::Incompleta(linha.to_string()));
    };
    // `ok_or_else`: a String do erro só é criada se o rank não existir
    let rank = ler_rank(rank).ok_or_else(|| ErroMural::Rank(rank.to_string()))?;
    Ok(Missao {
        nome: nome.to_string(),
        rank,
        recompensa: recompensa.parse::<u32>()?,
        local: local.to_string(),
    })
}

// Tudo ou nada: no primeiro Err, o `collect` para e devolve esse erro
fn ler_mural(texto: &str) -> Result<Vec<Missao>, ErroMural> {
    texto.lines().map(ler_missao).collect()
}

// --- 3. Filtros guardados ---------------------------------

// Cada filtro é uma closure numa caixa: closures diferentes no mesmo array
type Filtro = Box<dyn Fn(&Missao) -> bool>;

fn rank_minimo(minimo: Rank) -> Filtro {
    Box::new(move |m| m.rank >= minimo)
}

fn paga_ao_menos(ouro: u32) -> Filtro {
    Box::new(move |m| m.recompensa >= ouro)
}

fn fora_de(local: &str) -> Filtro {
    let local = local.to_string(); // a caixa precisa ser dona do texto, não emprestar
    Box::new(move |m| m.local != local)
}

// Passa só a missão que todos os filtros aprovam
fn filtrar<'a>(mural: &'a [Missao], filtros: &[Filtro]) -> Vec<&'a Missao> {
    mural
        .iter()
        .filter(|&m| filtros.iter().all(|filtro| filtro(m)))
        .collect()
}

// --- 4. O rodízio do grupo ---------------------------------

// Iterador sem fim: Frieren, Fern, Stark, Frieren, Fern... (o `cycle()` da std, à mão)
struct Rodizio<'a> {
    grupo: &'a [&'a str],
    vez: usize,
}

impl<'a> Rodizio<'a> {
    fn new(grupo: &'a [&'a str]) -> Self {
        Self { grupo, vez: 0 }
    }
}

impl<'a> Iterator for Rodizio<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let nome = self.grupo.get(self.vez).copied()?; // grupo vazio: None logo de cara
        self.vez = (self.vez + 1) % self.grupo.len(); // volta para 0 depois do último
        Some(nome)
    }
}

// --- 5. Relatório e diário ---------------------------------

fn relatorio(mural: &[Missao]) {
    let ouro: u32 = mural.iter().map(|m| m.recompensa).sum();
    println!("{} missões, {ouro} de ouro no total", mural.len());

    if let Some(maior) = mural.iter().max_by_key(|m| m.recompensa) {
        println!("Mais bem paga: {maior}");
    }
    if let Some(primeira) = mural.iter().find(|m| m.local == "Floresta") {
        println!("Primeira da Floresta: {primeira}");
    }
    let tem_s = mural.iter().any(|m| m.rank == Rank::S);
    println!("Tem rank S? {tem_s}");

    // Contar por local: `fold` com um HashMap como acumulado (o `entry` do Módulo 5)
    let contagem = mural.iter().fold(HashMap::new(), |mut mapa, m| {
        *mapa.entry(m.local.as_str()).or_insert(0) += 1;
        mapa
    });
    // A ordem do HashMap muda a cada execução: vira Vec e é ordenado antes de imprimir
    let mut locais: Vec<(&str, u32)> = contagem.into_iter().collect();
    locais.sort();
    let partes: Vec<String> = locais
        .iter()
        .map(|(local, n)| format!("{local} {n}"))
        .collect();
    println!("Por local: {}", partes.join(", "));
}

// O `buscar_sem_caixa` da Aula 3
fn buscar<'a>(consulta: &str, texto: &'a str) -> Vec<&'a str> {
    let consulta = consulta.to_lowercase();
    texto
        .lines()
        .filter(|linha| linha.to_lowercase().contains(&consulta))
        .collect()
}

// --- 6. A guilda abre o mural ---------------------------------

fn main() {
    // Uma linha ruim derruba o mural inteiro
    match ler_mural("Caçar slimes | C | 30 | Floresta\nColher ervas | C | quinze | Floresta") {
        Ok(_) => println!("Mural aceito"),
        Err(erro) => println!("Mural recusado: {erro}"),
    }

    let texto = "Caçar slimes | C | 30 | Floresta\n\
        Escoltar o mercador | B | 80 | Estrada\n\
        Selar o Mímico | B | 120 | Masmorra\n\
        Derrotar o golem | A | 200 | Masmorra\n\
        Colher ervas | C | 15 | Floresta\n\
        Proteger a vila | B | 60 | Vila\n\
        Explorar as ruínas | A | 300 | Ruínas\n\
        Enfrentar o dragão | S | 900 | Montanha";
    let mural = match ler_mural(texto) {
        Ok(mural) => mural,
        Err(erro) => {
            println!("Mural recusado: {erro}");
            return;
        }
    };

    println!("\n== Relatório ==");
    relatorio(&mural);

    // Rank B ou mais, 50 de ouro ou mais e nada na Montanha (o dragão fica para depois)
    let filtros = [rank_minimo(Rank::B), paga_ao_menos(50), fora_de("Montanha")];
    let mut escolhidas = filtrar(&mural, &filtros);
    escolhidas.sort_by_key(|m| Reverse(m.recompensa));

    // O rodízio não tem fim; o `zip` para quando as missões acabam
    let grupo = ["Frieren", "Fern", "Stark"];
    let diario: Vec<String> = escolhidas
        .iter()
        .zip(Rodizio::new(&grupo))
        .enumerate()
        .map(|(i, (missao, nome))| format!("{}. {nome} aceitou {missao}", i + 1))
        .collect();
    let diario = diario.join("\n");
    println!("\n== Diário ==\n{diario}");

    println!("\n== Busca: \"FRIEREN\" ==");
    for linha in buscar("FRIEREN", &diario) {
        println!("{linha}");
    }
}

// --- 7. Testes: lendo o mural ---------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Mural pequeno, só para os testes: uma missão de cada rank
    const MURAL: &str = "Caçar slimes | C | 30 | Floresta\n\
        Selar o Mímico | B | 120 | Masmorra\n\
        Derrotar o golem | A | 200 | Masmorra\n\
        Enfrentar o dragão | S | 900 | Montanha";

    // Ajudantes: funções sem `#[test]` dentro do módulo de testes
    fn missao(nome: &str, rank: Rank, recompensa: u32, local: &str) -> Missao {
        Missao {
            nome: nome.to_string(),
            rank,
            recompensa,
            local: local.to_string(),
        }
    }

    // `expect` pode aqui: se o mural de teste não ler, o bug é do próprio teste
    fn mural() -> Vec<Missao> {
        ler_mural(MURAL).expect("o mural de teste é válido")
    }

    #[test]
    fn le_a_missao_e_apara_os_espacos() {
        let lida = ler_missao("  Selar o Mímico |B|  120 | Masmorra ");
        assert_eq!(lida, Ok(missao("Selar o Mímico", Rank::B, 120, "Masmorra")));
    }

    #[test]
    fn recusa_linha_incompleta() {
        let linha = "Colher ervas | C | 15";
        let esperado = Err(ErroMural::Incompleta(linha.to_string()));
        assert_eq!(ler_missao(linha), esperado);
    }

    #[test]
    fn recusa_rank_desconhecido() {
        let esperado = Err(ErroMural::Rank("Z".to_string()));
        assert_eq!(ler_missao("Caçar ratos | Z | 10 | Vila"), esperado);
    }

    #[test]
    fn recusa_recompensa_que_nao_e_numero() {
        let lida = ler_missao("Colher ervas | C | quinze | Floresta");
        assert!(matches!(lida, Err(ErroMural::Recompensa(_))));
    }

    #[test]
    fn le_o_mural_inteiro_em_ordem() -> Result<(), ErroMural> {
        let mural = ler_mural(MURAL)?;
        assert_eq!(mural.len(), 4);
        let dragao = missao("Enfrentar o dragão", Rank::S, 900, "Montanha");
        assert_eq!(mural[3], dragao);
        Ok(())
    }

    // Tudo ou nada: a primeira linha é boa, mas a segunda derruba o mural
    #[test]
    fn uma_linha_ruim_derruba_o_mural() {
        let texto = "Caçar slimes | C | 30 | Floresta\nColher ervas | C | quinze | Floresta";
        assert!(matches!(ler_mural(texto), Err(ErroMural::Recompensa(_))));
    }

    // Borda: texto vazio não tem linhas, então não tem linha ruim
    #[test]
    fn mural_vazio_e_valido() {
        assert_eq!(ler_mural(""), Ok(Vec::new()));
    }

    #[test]
    fn ranks_seguem_a_ordem_das_variantes() {
        assert!(Rank::C < Rank::B && Rank::B < Rank::A && Rank::A < Rank::S);
    }

    // --- 8. Testes: filtros ---------------------------------

    // Os nomes das missões que passaram, para comparar com um array de `&str`
    fn nomes<'a>(missoes: &[&'a Missao]) -> Vec<&'a str> {
        missoes.iter().map(|m| m.nome.as_str()).collect()
    }

    // A caixa com a closure se chama como uma função: `filtro(&missao)`
    #[test]
    fn cada_filtro_sozinho() {
        let mural = mural();
        let (slimes, dragao) = (&mural[0], &mural[3]);

        let rank_a = rank_minimo(Rank::A);
        assert!(rank_a(dragao) && !rank_a(slimes));

        let cem = paga_ao_menos(100);
        assert!(cem(dragao) && !cem(slimes));

        let sem_montanha = fora_de("Montanha");
        assert!(sem_montanha(slimes) && !sem_montanha(dragao));
    }

    #[test]
    fn filtrar_exige_todos_os_filtros() {
        let mural = mural();
        let mut filtros = vec![rank_minimo(Rank::B), paga_ao_menos(150)];
        let passaram = nomes(&filtrar(&mural, &filtros));
        assert_eq!(passaram, ["Derrotar o golem", "Enfrentar o dragão"]);

        // Um filtro a mais só pode tirar missões da lista, nunca pôr
        filtros.push(fora_de("Montanha"));
        assert_eq!(nomes(&filtrar(&mural, &filtros)), ["Derrotar o golem"]);
    }

    // Borda: `all` numa lista vazia é `true` (verdade por vacuidade), então tudo passa
    #[test]
    fn sem_filtros_passa_tudo() {
        let mural = mural();
        assert_eq!(filtrar(&mural, &[]).len(), mural.len());
    }

    // --- 9. Testes: rodízio e busca ---------------------------------

    #[test]
    fn rodizio_volta_ao_comeco() {
        let grupo = ["Frieren", "Fern", "Stark"];
        let vezes: Vec<&str> = Rodizio::new(&grupo).take(5).collect();
        assert_eq!(vezes, ["Frieren", "Fern", "Stark", "Frieren", "Fern"]);
    }

    // Borda: o `?` do `next` devolve `None` antes do `%` dividir por zero
    #[test]
    fn rodizio_de_grupo_vazio_termina_logo() {
        let vazio: [&str; 0] = [];
        assert_eq!(Rodizio::new(&vazio).next(), None);
    }

    // Devolve a linha original (com a caixa dela), mesmo buscando em maiúsculas
    #[test]
    fn busca_ignora_maiusculas_e_minusculas() {
        let achadas = buscar("MÍMICO", MURAL);
        assert_eq!(achadas, ["Selar o Mímico | B | 120 | Masmorra"]);
    }

    #[test]
    fn busca_sem_resultado_devolve_vazio() {
        assert!(buscar("Himmel", MURAL).is_empty());
    }
}
