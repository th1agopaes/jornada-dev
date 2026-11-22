//! MC2 · Rust do zero — Módulo 6 · Aula 2: o módulo `monstro` (`monstro.rs`)
//! Objetivo: o tipo `Monstro`, que usa o `Item` do módulo irmão e tem um submódulo.
//! Declarado no `main.rs` com `mod monstro;`.

// Submódulo em pasta: o conteúdo de `chefe` fica em `monstro/chefe.rs`
pub mod chefe;

// O `mod item;` já foi escrito no `main.rs`; aqui só se cria o atalho
use crate::item::Item;

pub struct Monstro {
    pub nome: String,
    vida: u32,
    recompensa: Option<Item>, // Option porque ela sai uma vez só (o `take` do Módulo 4)
}

impl Monstro {
    pub fn new(nome: &str, vida: u32, recompensa: Item) -> Self {
        Monstro {
            nome: nome.to_string(),
            vida,
            recompensa: Some(recompensa),
        }
    }

    pub fn vida(&self) -> u32 {
        self.vida
    }

    // Devolve a recompensa só no golpe que derruba; depois disso, o `take` já deixou None
    pub fn receber_dano(&mut self, dano: u32) -> Option<Item> {
        self.vida = self.vida.saturating_sub(dano);
        if self.vida == 0 {
            self.recompensa.take()
        } else {
            None
        }
    }
}
