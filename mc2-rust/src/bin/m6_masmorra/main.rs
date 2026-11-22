//! MC2 · Rust do zero — Módulo 6 · Aula 2: módulos em arquivos
//! Objetivo: dividir um executável em vários arquivos e entender pacote × crate.
//! Rodar:    `cargo run --bin m6_masmorra`

// `mod nome;` (com `;` no lugar das chaves) = "o conteúdo deste módulo está em nome.rs"
mod item;
mod monstro;

// `use` no topo: atalhos que valem para o arquivo inteiro
use item::Item;
use monstro::{Monstro, chefe};

// --- 1. Um tipo que veio de outro arquivo  ----------------------

fn bau() {
    let tesouro = [Item::Pocao(30), Item::Ouro(250)];
    for item in &tesouro {
        println!("No baú: {}", item.descricao());
    }
    // -> No baú: poção (+30 de vida)
    // -> No baú: 250 moedas de ouro
}

// --- 2. Um módulo usando outro  ----------------------

fn combate() {
    let mut slime = Monstro::new("Slime", 30, Item::Pocao(20));
    for dano in [12, 12, 12] {
        match slime.receber_dano(dano) {
            Some(item) => println!("{} caiu e deixou: {}", slime.nome, item.descricao()),
            None => println!("{} tem {} de vida", slime.nome, slime.vida()),
        }
    }
    // -> Slime tem 18 de vida
    // -> Slime tem 6 de vida
    // -> Slime caiu e deixou: poção (+20 de vida)
}

// --- 3. Submódulo em pasta  ----------------------

fn chefe_final() {
    let mut dragao = chefe::dragao(); // caminho inteiro: crate::monstro::chefe::dragao
    println!("Chefe: {} ({} de vida)", dragao.nome, dragao.vida());
    // -> Chefe: Dragão Dourado (120 de vida)

    if let Some(item) = dragao.receber_dano(999) {
        println!("Recompensa: {}", item.descricao()); // -> Recompensa: grimório de Zoltraak
    }
}

fn main() {
    bau();
    combate();
    chefe_final();
}
