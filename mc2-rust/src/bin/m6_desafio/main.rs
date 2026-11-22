//! MC2 · Rust do zero — Módulo 6 · Exercício: o inventário da Frieren em módulos
//! Objetivo: dividir o `m5_desafio` em módulos e acrescentar drops aleatórios com semente.
//! Rodar:    `cargo run --bin m6_desafio`

mod comando;
mod inventario;
mod loot;

use inventario::Inventario;

const SEMENTE: u64 = 42;

fn main() {
    let comandos = [
        "pegar poção 3",
        "pegar pena 0",
        "usar pena",
        "pegar elixir",
        "PEGAR grimório 1",
        "usar poção",
        "usar elixir",
        "usar elixir",
        "descartar poção 5",
        "descartar poção",
        "pegar maçã dez",
        "pegar",
        "dançar",
        "",
        "   pegar   maçã    4",
        "explorar",
        "explorar",
        "usar pena",
        "explorar",
        "explorar",
        "usar elixir",
    ];

    let mut jogo = Inventario::new(SEMENTE);
    for linha in comandos {
        if let Err(erro) = jogo.executar(linha) {
            println!("erro em {linha:?}: {}", erro.mensagem());
        }
    }

    println!("\nDiário:");
    for (i, evento) in jogo.diario().iter().enumerate() {
        println!("{}. {evento}", i + 1);
    }

    println!("\nInventário:");
    for (item, qtd) in jogo.listar() {
        println!("- {item}: {qtd}");
    }
}
