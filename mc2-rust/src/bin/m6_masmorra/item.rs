//! MC2 · Rust do zero — Módulo 6 · Aula 2: o módulo `item` (`item.rs`)
//! Objetivo: guardar o tipo `Item`, o drop dos monstros, num arquivo próprio.
//! Declarado no `main.rs` com `mod item;`.

// `pub` porque quem usa o `Item` (`main.rs` e `monstro.rs`) está fora deste módulo
pub enum Item {
    Pocao(u32),
    Ouro(u32),
    Grimorio(String),
}

impl Item {
    pub fn descricao(&self) -> String {
        match self {
            Item::Pocao(cura) => format!("poção (+{cura} de vida)"),
            Item::Ouro(qtd) => format!("{qtd} moedas de ouro"),
            Item::Grimorio(magia) => format!("grimório de {magia}"),
        }
    }
}
