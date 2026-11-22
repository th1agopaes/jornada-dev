//! MC2 · Rust do zero — Módulo 6 · Exercício: o módulo `inventario` (`inventario.rs`)
//! Objetivo: guardar os itens e o diário, e executar os comandos.

use std::collections::HashMap;

use rand::SeedableRng;
use rand::rngs::ChaCha8Rng;

use crate::comando::{self, Comando, ErroJogo};
use crate::loot;

// --- 1. O inventário ---------------------------------

pub struct Inventario {
    itens: HashMap<String, u32>,
    diario: Vec<String>,
    rng: ChaCha8Rng,
}

impl Inventario {
    pub fn new(semente: u64) -> Self {
        Inventario {
            itens: HashMap::new(),
            diario: Vec::new(),
            rng: ChaCha8Rng::seed_from_u64(semente),
        }
    }

    fn guardar(&mut self, item: String, qtd: u32) {
        let total = self.itens.entry(item).or_insert(0);
        *total += qtd;
    }

    fn pegar(&mut self, item: String, qtd: u32) {
        self.diario.push(format!("pegou {qtd}x {item}"));
        self.guardar(item, qtd);
    }

    fn explorar(&mut self) {
        let (item, qtd) = loot::sortear(&mut self.rng);
        // o diário vem antes porque o guardar move a String
        self.diario.push(format!("explorou e achou {qtd}x {item}"));
        self.guardar(item, qtd);
    }

    fn usar(&mut self, item: &str) -> Result<(), ErroJogo> {
        let atual = self
            .itens
            .get_mut(item)
            .ok_or(ErroJogo::SemItem(item.to_string()))?;

        *atual -= 1;
        let restam = *atual;

        if restam == 0 {
            self.itens.remove(item);
        }

        self.diario.push(format!("usou {item} (restam {restam})"));
        Ok(())
    }

    fn descartar(&mut self, item: &str, qtd: u32) -> Result<(), ErroJogo> {
        let atual = self
            .itens
            .get_mut(item)
            .ok_or(ErroJogo::SemItem(item.to_string()))?;

        if *atual < qtd {
            return Err(ErroJogo::Insuficiente {
                item: item.to_string(),
                tem: *atual,
                pediu: qtd,
            });
        }

        *atual -= qtd;
        let restam = *atual;

        if restam == 0 {
            self.itens.remove(item);
        }

        self.diario.push(format!("descartou {qtd}x {item}"));
        Ok(())
    }

    pub fn executar(&mut self, linha: &str) -> Result<(), ErroJogo> {
        match comando::interpretar(linha)? {
            Comando::Pegar { item, qtd } => self.pegar(item, qtd),
            Comando::Usar(item) => self.usar(&item)?,
            Comando::Descartar { item, qtd } => self.descartar(&item, qtd)?,
            Comando::Explorar => self.explorar(),
        }
        Ok(())
    }

    pub fn listar(&self) -> Vec<(&str, u32)> {
        let mut lista = Vec::new();
        for (item, qtd) in &self.itens {
            lista.push((item.as_str(), *qtd));
        }
        lista.sort();
        lista
    }

    // o campo é privado para que ninguém de fora escreva no diário,
    // e o getter só dá leitura
    pub fn diario(&self) -> &[String] {
        &self.diario
    }
}
