//! MC2 · Rust do zero — Módulo 6 · Aula 2: o submódulo `monstro::chefe` (`monstro/chefe.rs`)
//! Objetivo: um submódulo em pasta, que usa o tipo do módulo pai com `super::`.
//! Declarado no `monstro.rs` com `pub mod chefe;`.

use super::Monstro; // o pai é `monstro`: o `super::` da Aula 1, agora entre arquivos
use crate::item::Item;

pub fn dragao() -> Monstro {
    let magia = String::from("Zoltraak");
    Monstro::new("Dragão Dourado", 120, Item::Grimorio(magia))
}
