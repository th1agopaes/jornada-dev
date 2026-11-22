//! MC2 · Rust do zero — Módulo 6 · Aula 3: a primeira dependência (`rand`)
//! Objetivo: usar um crate do crates.io para rolar dados, sortear drops e repetir sorteios.
//! Rodar:    `cargo run --bin m6_rand`

// `rand` não vem no prelude: o caminho começa pelo nome do crate, como em `std::...`
use rand::rngs::ChaCha8Rng; // gerador com semente (seção 4; liberado pela feature `chacha`)
use rand::seq::IndexedRandom; // trait que dá o `choose` às fatias (seção 3)
use rand::{RngExt, SeedableRng}; // traits dos métodos `random_*` e do `seed_from_u64`

// --- 1. O gerador e o dado ---------------------------

fn dados() {
    // `rand::rng()` = gerador da thread autal, com semente tirada do sistema operacional
    let mut rng = rand::rng();
    let d6: u32 = rng.random_range(1..=6); // o mesmo `1..=6` do `for`: de 1 a 6, inclusive
    let d20: u32 = rng.random_range(1..=20);
    println!("d6 = {d6}, d20 = {d20}"); // -> muda a cada execução
}

// --- 2. Chance de crítico ---------------------------

fn criticos() {
    let mut rng = rand::rng();
    let mut total = 0;

    print!("Golpes:");
    for _ in 0..8 {
        // `random_bool(p)`: true com probabilidade p (0.25 = 25%)
        if rng.random_bool(0.25) {
            total += 1;
            print!(" X");
        } else {
            print!(" .");
        }
    }
    println!(" | críticos: {total}"); // -> Golpes: . X . . . . X . | críticos: 2
}

// --- 3. Tabela de drop ---------------------------

fn tabela_de_drop() {
    let mut rng = rand::rng();

    // Raridade por faixa: 1 a 60 comum (60%), 61 a 90 raro (30%), 91 a 100 lendário (10%)
    let sorteio: u32 = rng.random_range(1..=100);
    let raridade = match sorteio {
        1..=60 => "comum",
        61..=90 => "raro",
        _ => "lendário", // 91..=100: o `match` não sabe que o sorteio para em 100
    };

    // `choose` sorteia um item da fatia e devolve Option<&T>: a fatia poderia estar vazia
    let itens = ["poção", "elixir", "grimório", "pena da fênix"];
    if let Some(item) = itens.choose(&mut rng) {
        println!("Drop: {item} ({raridade}, sorteio {sorteio})");
        // -> Drop: elixir (raro, ...)
    }
}

// --- 4. Semente: o mesmo sorteio de novo ---------------------------

fn semente() {
    // Dois geradores com a mesma semente (seed) produzem a mesma sequência
    let mut mundo_a = ChaCha8Rng::seed_from_u64(2026);
    let mut mundo_b = ChaCha8Rng::seed_from_u64(2026);

    let mut rolagens_a: Vec<u32> = Vec::new();
    let mut rolagens_b: Vec<u32> = Vec::new();
    for _ in 0..5 {
        rolagens_a.push(mundo_a.random_range(1..=20));
        rolagens_b.push(mundo_b.random_range(1..=20));
    }

    println!("Mundo A: {rolagens_a:?}"); // -> Mundo A: [2, 12, 7, 6, 15]
    println!("Mundo B: {rolagens_b:?}"); // -> Mundo B: [2, 12, 7, 6, 15]
    println!("Iguais? {}", rolagens_a == rolagens_b); // -> Iguais? true
}

fn main() {
    dados();
    criticos();
    tabela_de_drop();
    semente();
}
