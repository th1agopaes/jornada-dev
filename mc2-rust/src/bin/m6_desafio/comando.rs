//! MC2 · Rust do zero — Módulo 6 · Exercício: o módulo `comando` (`comando.rs`)
//! Objetivo: transformar a linha digitada num `Comando` ou num `ErroJogo`.

// --- 1. Comandos e erros ---------------------------------

pub enum Comando {
    Pegar { item: String, qtd: u32 },
    Usar(String),
    Descartar { item: String, qtd: u32 },
    Explorar,
}

pub enum ErroJogo {
    Vazio,
    Desconhecido(String),
    FaltaItem(String),
    QtdInvalida(String),
    SemItem(String),
    Insuficiente { item: String, tem: u32, pediu: u32 },
}

impl ErroJogo {
    pub fn mensagem(&self) -> String {
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

pub fn interpretar(linha: &str) -> Result<Comando, ErroJogo> {
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
        "explorar" => Ok(Comando::Explorar),
        outro => Err(ErroJogo::Desconhecido(outro.to_string())),
    }
}
