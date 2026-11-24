//! MC2 · Rust do zero — Módulo 9 · Aula 2: testando erros, panics, sorteios e esteiras
//! Objetivo: testes com `Result`, `#[should_panic]`, semente, `#[ignore]` e iterador sem fim.
//! Rodar:    `cargo run --bin m9_testes_jogo`
//! Testar:   `cargo test --bin m9_testes_jogo` (o demorado: `-- --ignored`)

use std::num::ParseIntError;

use rand::rngs::ChaCha8Rng;
use rand::{Rng, RngExt, SeedableRng};

// --- 1. Lendo um golpe ---------------------------------

#[derive(Debug, PartialEq)]
enum Elemento {
    Fogo,
    Raio,
}

#[derive(Debug, PartialEq)]
struct Golpe {
    elemento: Elemento,
    poder: u32,
}

// `Debug` + `PartialEq` também no erro: o teste compara e imprime o `Err` inteiro
#[derive(Debug, PartialEq)]
enum ErroGolpe {
    Incompleto,
    Elemento(String),
    Poder(ParseIntError),
}

impl From<ParseIntError> for ErroGolpe {
    fn from(erro: ParseIntError) -> Self {
        ErroGolpe::Poder(erro)
    }
}

// "fogo 30" -> Golpe { elemento: Fogo, poder: 30 }
fn ler_golpe(texto: &str) -> Result<Golpe, ErroGolpe> {
    let mut partes = texto.split_whitespace();
    let (Some(elemento), Some(poder)) = (partes.next(), partes.next()) else {
        return Err(ErroGolpe::Incompleto);
    };
    let elemento = match elemento {
        "fogo" => Elemento::Fogo,
        "raio" => Elemento::Raio,
        outro => return Err(ErroGolpe::Elemento(outro.to_string())),
    };
    Ok(Golpe {
        elemento,
        poder: poder.parse::<u32>()?,
    })
}

// --- 2. A mochila ---------------------------------

struct Mochila {
    capacidade: usize,
    itens: Vec<String>,
}

impl Mochila {
    // Capacidade 0 é bug de quem programou, não erro do jogador: panic, não `Result`
    fn new(capacidade: usize) -> Self {
        assert!(capacidade > 0, "a mochila precisa de ao menos 1 espaço");
        Self {
            capacidade,
            itens: Vec::new(),
        }
    }

    // Cheia: o item volta no `Err`, em vez de sumir
    fn guardar(&mut self, item: &str) -> Result<(), String> {
        if self.itens.len() == self.capacidade {
            return Err(item.to_string());
        }
        self.itens.push(item.to_string());
        Ok(())
    }
}

// --- 3. O drop sorteado ---------------------------------

#[derive(Debug, PartialEq)]
enum Raridade {
    Comum,
    Rara,
    Lendaria,
}

// 70% comum, 25% rara, 5% lendária; o gerador vem de fora (injeção de dependência, Módulo 7)
fn sortear_drop(rng: &mut impl Rng) -> Raridade {
    let rolagem: u32 = rng.random_range(1..=100);
    match rolagem {
        1..=70 => Raridade::Comum,
        71..=95 => Raridade::Rara,
        _ => Raridade::Lendaria,
    }
}

// --- 4. As ondas de inimigos ---------------------------------

// Esteira sem fim: 3, 5, 7, 9... inimigos por onda
struct Ondas {
    inimigos: u32,
}

impl Iterator for Ondas {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let atual = self.inimigos;
        self.inimigos += 2;
        Some(atual)
    }
}

fn ondas() -> Ondas {
    Ondas { inimigos: 3 }
}

// --- 5. O programa ---------------------------------

fn main() {
    if let Ok(golpe) = ler_golpe("raio 45") {
        println!("{golpe:?}"); // -> Golpe { elemento: Raio, poder: 45 }
    }
    if let Err(erro) = ler_golpe("gelo 10") {
        println!("Recusado: {erro:?}"); // -> Recusado: Elemento("gelo")
    }

    let mut mochila = Mochila::new(1);
    let _ = mochila.guardar("poção");
    if let Err(item) = mochila.guardar("éter") {
        println!("Mochila cheia, {item} ficou no chão"); // -> Mochila cheia, éter ficou no chão
    }

    // O mesmo `sortear_drop` aceita o gerador com semente e o do sistema
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let drops: Vec<Raridade> = (0..5).map(|_| sortear_drop(&mut rng)).collect();
    println!("Semente 42: {drops:?}");
    println!("Sem semente: {:?}", sortear_drop(&mut rand::rng())); // -> muda a cada execução

    let primeiras: Vec<u32> = ondas().take(5).collect();
    println!("Ondas: {primeiras:?}"); // -> Ondas: [3, 5, 7, 9, 11]
}

// --- 6. Testes de `Result` ---------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_um_golpe_valido() {
        let esperado = Golpe {
            elemento: Elemento::Fogo,
            poder: 30,
        };
        assert_eq!(ler_golpe("fogo 30"), Ok(esperado));
    }

    #[test]
    fn recusa_elemento_desconhecido() {
        let esperado = Err(ErroGolpe::Elemento("gelo".to_string()));
        assert_eq!(ler_golpe("gelo 10"), esperado);
    }

    #[test]
    fn recusa_golpe_incompleto() {
        assert_eq!(ler_golpe("fogo"), Err(ErroGolpe::Incompleto));
    }

    // `ParseIntError` não se monta à mão: `matches!` confere só a variante
    #[test]
    fn recusa_poder_que_nao_e_numero() {
        assert!(matches!(ler_golpe("fogo muito"), Err(ErroGolpe::Poder(_))));
    }

    // Teste que devolve `Result`: um `Err` no `?` reprova o teste
    #[test]
    fn soma_o_poder_de_dois_golpes() -> Result<(), ErroGolpe> {
        let fogo = ler_golpe("fogo 30")?;
        let raio = ler_golpe("raio 45")?;
        assert_eq!(fogo.poder + raio.poder, 75);
        Ok(())
    }

    // --- 7. Testes de panic ---------------------------------

    // Passa só se der panic, e com um texto que contém o `expected`
    #[test]
    #[should_panic(expected = "ao menos 1 espaço")]
    fn mochila_sem_espaco_e_bug() {
        Mochila::new(0);
    }

    #[test]
    fn mochila_cheia_devolve_o_item() {
        let mut mochila = Mochila::new(2);
        assert_eq!(mochila.guardar("poção"), Ok(()));
        assert_eq!(mochila.guardar("éter"), Ok(()));
        assert_eq!(mochila.guardar("elixir"), Err("elixir".to_string()));
        assert_eq!(mochila.itens, ["poção", "éter"]); // `Vec<String>` compara com `[&str; 2]`
    }

    // --- 8. Sorteios com semente ---------------------------------

    #[test]
    fn mesma_semente_mesmos_drops() {
        let mut a = ChaCha8Rng::seed_from_u64(7);
        let mut b = ChaCha8Rng::seed_from_u64(7);
        let drops_a: Vec<Raridade> = (0..50).map(|_| sortear_drop(&mut a)).collect();
        let drops_b: Vec<Raridade> = (0..50).map(|_| sortear_drop(&mut b)).collect();
        assert_eq!(drops_a, drops_b);
    }

    // Teste de regressão: se a tabela de drop mudar sem querer, a sequência muda e o teste avisa
    #[test]
    fn semente_42_da_a_sequencia_conhecida() {
        use Raridade::{Comum, Lendaria, Rara};
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let drops: Vec<Raridade> = (0..5).map(|_| sortear_drop(&mut rng)).collect();
        assert_eq!(drops, [Comum, Comum, Comum, Lendaria, Rara]);
    }

    // Dez milhões de sorteios: lento no perfil de teste, então só roda quando pedido
    #[test]
    #[ignore = "demorado: 10 milhões de sorteios"]
    fn lendarias_saem_perto_de_5_por_cento() {
        let mut rng = ChaCha8Rng::seed_from_u64(2026);
        let total = 10_000_000;
        let lendarias = (0..total)
            .filter(|_| sortear_drop(&mut rng) == Raridade::Lendaria)
            .count();
        let taxa = lendarias as f64 / total as f64;
        println!("taxa de lendárias: {taxa:.4}"); // só aparece com `--show-output`
        assert!((0.045..0.055).contains(&taxa), "taxa fora da faixa: {taxa}");
    }

    // --- 9. Esteiras sem fim ---------------------------------

    // `take` põe fim na esteira; sem ele, o `collect` nunca terminaria
    #[test]
    fn ondas_crescem_de_2_em_2() {
        let primeiras: Vec<u32> = ondas().take(4).collect();
        assert_eq!(primeiras, [3, 5, 7, 9]);
    }

    #[test]
    fn decima_onda_tem_21_inimigos() {
        assert_eq!(ondas().nth(9), Some(21)); // `nth` conta do 0
    }
}
