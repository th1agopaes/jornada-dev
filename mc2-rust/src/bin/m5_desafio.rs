//! MC2 · Rust do zero — Módulo 5 · Exercício: o inventário da Frieren
//! Objetivo: ler comandos de texto, guardar itens num `HashMap`, registrar eventos num `Vec`
//! e tratar cada falha com `Result` e `?`.
//! Rodar:    `cargo run --bin m5_desafio`

use std::collections::HashMap;

// --- 1. Comandos e erros ---------------------------------

enum Comando {
    Pegar { item: String, qtd: u32 },
    Usar(String),
    Descartar { item: String, qtd: u32 },
}

enum ErroJogo {
    Vazio,
    Desconhecido(String),
    FaltaItem(String),
    QtdInvalida(String),
    SemItem(String),
    Insuficiente { item: String, tem: u32, pediu: u32 },
}

impl ErroJogo {
    fn mensagem(&self) -> String {
        match self {
            ErroJogo::Vazio => String::from("digite um comando"),
            ErroJogo::Desconhecido(verbo) => format!("comando desconhecido: {verbo}"),
            ErroJogo::FaltaItem(verbo) => format!("`{verbo}` precisa de um item"),
            ErroJogo::QtdInvalida(texto) => format!("`{texto}` não é uma quantidade válida"),
            ErroJogo::SemItem(item) => format!("não tem {item} no inventário"),
            ErroJogo::Insuficiente { item, tem, pediu } => {
                format!("só tem {tem} de {item}, pediu {pediu}")
            }
        }
    }
}

// --- 2. Lendo o comando ---------------------------------

fn ler_item(verbo: &str, palavra: Option<&str>) -> Result<String, ErroJogo> {
    let item = palavra.ok_or(ErroJogo::FaltaItem(verbo.to_string()))?;
    Ok(item.to_string())
}

fn ler_qtd(palavra: Option<&str>) -> Result<u32, ErroJogo> {
    let Some(texto) = palavra else {
        return Ok(1);
    };
    let Ok(qtd) = texto.parse::<u32>() else {
        return Err(ErroJogo::QtdInvalida(texto.to_string()));
    };
    if qtd == 0 {
        return Err(ErroJogo::QtdInvalida(texto.to_string()));
    }
    Ok(qtd)
}

fn interpretar(linha: &str) -> Result<Comando, ErroJogo> {
    let limpa = linha.trim().to_lowercase();
    let mut partes = limpa.split_whitespace();
    let verbo = partes.next().ok_or(ErroJogo::Vazio)?;

    match verbo {
        "pegar" => {
            let item = ler_item(verbo, partes.next())?;
            let qtd = ler_qtd(partes.next())?;
            Ok(Comando::Pegar { item, qtd })
        }
        "descartar" => {
            let item = ler_item(verbo, partes.next())?;
            let qtd = ler_qtd(partes.next())?;
            Ok(Comando::Descartar { item, qtd })
        }
        "usar" => {
            let item = ler_item(verbo, partes.next())?;
            Ok(Comando::Usar(item))
        }
        outro => Err(ErroJogo::Desconhecido(outro.to_string())),
    }
}

// --- 3. O inventário ---------------------------------

struct Inventario {
    itens: HashMap<String, u32>,
    diario: Vec<String>,
}

impl Inventario {
    fn new() -> Self {
        Inventario {
            itens: HashMap::new(),
            diario: Vec::new(),
        }
    }

    fn pegar(&mut self, item: String, qtd: u32) {
        self.diario.push(format!("pegou {qtd}x {item}"));
        let total = self.itens.entry(item).or_insert(0);
        *total += qtd;
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

    fn executar(&mut self, linha: &str) -> Result<(), ErroJogo> {
        match interpretar(linha)? {
            Comando::Pegar { item, qtd } => self.pegar(item, qtd),
            Comando::Usar(item) => self.usar(&item)?,
            Comando::Descartar { item, qtd } => self.descartar(&item, qtd)?,
        }
        Ok(())
    }

    fn listar(&self) -> Vec<(&str, u32)> {
        let mut lista = Vec::new();
        for (item, qtd) in &self.itens {
            lista.push((item.as_str(), *qtd));
        }
        lista.sort();
        lista
    }
}

// --- 4. Rodando a aventura ---------------------------------

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
    ];

    let mut jogo = Inventario::new();
    for linha in comandos {
        if let Err(erro) = jogo.executar(linha) {
            println!("erro em {linha:?}: {}", erro.mensagem());
        }
    }

    println!("\nDiário:");
    for (i, evento) in jogo.diario.iter().enumerate() {
        println!("{}. {evento}", i + 1);
    }

    println!("\nInventário:");
    for (item, qtd) in jogo.listar() {
        println!("- {item}: {qtd}");
    }
}
