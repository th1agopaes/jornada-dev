//! MC2 · Rust do zero — Módulo 6 · Exercício: o módulo `loot` (`loot.rs`)
//! Objetivo: sortear o que a Frieren acha ao explorar, pela tabela de drop.

use rand::RngExt;
use rand::rngs::ChaCha8Rng;
use rand::seq::IndexedRandom;

const COMUNS: [&str; 3] = ["poção", "maçã", "pena"];

pub fn sortear(rng: &mut ChaCha8Rng) -> (String, u32) {
    let rolagem: u32 = rng.random_range(1..=100);

    // cada faixa é uma chance (60% comum, 30% raro, 10% lendário)
    match rolagem {
        1..=60 => {
            let item = COMUNS.choose(rng).expect("a lista COMUNS nunca está vazia");
            let qtd = rng.random_range(1..=3);
            (item.to_string(), qtd)
        }
        61..=90 => (String::from("elixir"), 1),
        _ => (String::from("grimório"), 1), // 91..=100: o `match` não sabe que para em 100
    }
}
