//! MC2 · Rust do zero — Módulo 9 · Aula 3: o executável que usa a biblioteca
//! Objetivo: chamar o crate `mc2_rust` de um binário, como o `main.rs` do *minigrep*.
//! Rodar:    `cargo run --bin m9_dados`

use std::error::Error;

use rand::SeedableRng;
use rand::rngs::ChaCha8Rng;

use mc2_rust::Rolagem;

// --- 1. A mesa de RPG ---------------------------------

// `Box<dyn Error>`: aceita qualquer erro que implemente `Error` (o `main` do *minigrep*)
fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = ChaCha8Rng::seed_from_u64(20);
    for texto in ["1d20", "2d6+3", "4d6"] {
        let rolagem = Rolagem::ler(texto)?;
        let total = rolagem.rolar(&mut rng);
        let (min, max) = (rolagem.minimo(), rolagem.maximo());
        println!("{rolagem}: tirou {total} (de {min} a {max})");
    }

    if let Err(erro) = Rolagem::ler("2d0") {
        println!("Recusada: {erro}"); // -> Recusada: quantidade e faces precisam ser maiores que 0
    }
    Ok(())
}
