//! MC2 · Rust do zero — Módulo 7 · Exercício: o torneio da guilda
//! Objetivo: traits, genéricos, `derive`, `Display`, `From` e lifetimes num combate por turnos.
//! Rodar:    `cargo run --bin m7_desafio`

use std::fmt;
use std::num::ParseIntError;

use rand::rngs::ChaCha8Rng;
use rand::{Rng, RngExt, SeedableRng};

const SEMENTE: u64 = 7; // semente fixa: a saída é sempre a mesma

// --- 1. Raridade ---------------------------------

// Só o que o `>=` do `main` pede: PartialOrd, que exige PartialEq
#[derive(PartialEq, PartialOrd)]
enum Raridade {
    Comum,
    Elite,
    Chefe,
}

impl fmt::Display for Raridade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let texto = match self {
            Raridade::Comum => "comum",
            Raridade::Elite => "elite",
            Raridade::Chefe => "chefe",
        };
        write!(f, "{texto}")
    }
}

// --- 2. O contrato do combate ---------------------------------

trait Combatente {
    fn nome(&self) -> &str;
    fn vida(&self) -> u32;
    fn vida_mut(&mut self) -> &mut u32;
    fn ataque(&self) -> u32;

    // Os padrão só usam os obrigatórios, por isso valem para qualquer tipo
    fn esta_vivo(&self) -> bool {
        self.vida() > 0
    }

    fn receber(&mut self, dano: u32) {
        let vida = self.vida_mut();
        *vida = vida.saturating_sub(dano);
    }
}

struct Heroi {
    nome: String,
    vida: u32,
    ataque: u32,
}

impl Combatente for Heroi {
    fn nome(&self) -> &str {
        &self.nome
    }

    fn vida(&self) -> u32 {
        self.vida
    }

    fn vida_mut(&mut self) -> &mut u32 {
        &mut self.vida
    }

    fn ataque(&self) -> u32 {
        self.ataque
    }
}

struct Inimigo {
    nome: String,
    vida: u32,
    ataque: u32,
    raridade: Raridade,
}

impl Combatente for Inimigo {
    fn nome(&self) -> &str {
        &self.nome
    }

    fn vida(&self) -> u32 {
        self.vida
    }

    fn vida_mut(&mut self) -> &mut u32 {
        &mut self.vida
    }

    fn ataque(&self) -> u32 {
        self.ataque
    }
}

// "Golem [elite, 40 HP]"; o `{}` da raridade usa o Display dela
impl fmt::Display for Inimigo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}, {} HP]", self.nome, self.raridade, self.vida)
    }
}

// --- 3. A escalação: texto -> Inimigo ---------------------------------

#[derive(Debug)]
enum ErroArena {
    Incompleta(String),    // faltou palavra na linha (guarda a linha inteira)
    Numero(ParseIntError), // vida ou ataque que não é número
    Raridade(String),      // raridade fora da lista (guarda a palavra)
}

impl fmt::Display for ErroArena {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroArena::Incompleta(linha) => write!(f, "linha incompleta: {linha}"),
            ErroArena::Numero(erro) => write!(f, "número inválido ({erro})"),
            ErroArena::Raridade(texto) => write!(f, "raridade desconhecida: {texto}"),
        }
    }
}

// É este `from` que o `?` chama quando um `parse` falha dentro do `ler_inimigo`
impl From<ParseIntError> for ErroArena {
    fn from(erro: ParseIntError) -> Self {
        ErroArena::Numero(erro)
    }
}

// "Golem 40 6 elite" -> Inimigo { nome: "Golem", vida: 40, ataque: 6, raridade: Elite }
fn ler_inimigo(linha: &str) -> Result<Inimigo, ErroArena> {
    let mut partes = linha.split_whitespace();
    // A tupla chama os quatro `next()` da esquerda para a direita; se faltar um, cai no `else`
    let (Some(nome), Some(vida), Some(ataque), Some(raridade)) =
        (partes.next(), partes.next(), partes.next(), partes.next())
    else {
        return Err(ErroArena::Incompleta(linha.to_string()));
    };
    let raridade = match raridade {
        "comum" => Raridade::Comum,
        "elite" => Raridade::Elite,
        "chefe" => Raridade::Chefe,
        outra => return Err(ErroArena::Raridade(outra.to_string())),
    };
    Ok(Inimigo {
        nome: nome.to_string(),
        vida: vida.parse::<u32>()?, // não é número: o `?` devolve ErroArena::Numero
        ataque: ataque.parse::<u32>()?,
        raridade,
    })
}

// --- 4. Genéricos e lifetimes: o combate ---------------------------------

// O `T` da Aula 2 com o `'a` da Aula 4: o retorno vem de `a` ou de `b`
fn mais_perigoso<'a, T: Combatente>(a: &'a T, b: &'a T) -> &'a T {
    if a.ataque() >= b.ataque() { a } else { b }
}

fn golpe(atacante: &impl Combatente, alvo: &mut impl Combatente, rng: &mut impl Rng) {
    let dano = atacante.ataque() + rng.random_range(0..=3); // `random_range` vem do `RngExt`
    alvo.receber(dano);
    let nome = alvo.nome();
    let vida = alvo.vida();
    println!("  {} causa {dano} ({nome}: {vida} HP)", atacante.nome());
}

// Genérico nos dois lados: serve também para Inimigo contra Inimigo
fn duelo(heroi: &mut impl Combatente, inimigo: &mut impl Combatente, rng: &mut impl Rng) -> bool {
    loop {
        golpe(heroi, inimigo, rng); // o `&mut heroi` vira `&` sozinho: o `golpe` só lê o atacante
        if !inimigo.esta_vivo() {
            return true;
        }
        golpe(inimigo, heroi, rng);
        if !heroi.esta_vivo() {
            return false;
        }
    }
}

// --- 5. O torneio ---------------------------------

fn main() {
    // `\` no fim da linha continua o texto sem a quebra e sem os espaços da linha seguinte
    let escalacao = "Slime 20 3 comum\nMímico muitos 6 elite\nGolem 40 6 elite\n\
                           Fantasma 25\nLich 60 11 lendário\nDragão 70 10 chefe";

    let mut inimigos = Vec::new();
    for linha in escalacao.lines() {
        match ler_inimigo(linha) {
            Ok(inimigo) => inimigos.push(inimigo),
            Err(erro) => println!("Linha ignorada: {erro}"),
        }
    }

    let perigo = mais_perigoso(&inimigos[1], &inimigos[2]);
    println!("Mais perigoso entre Golem e Dragão: {}", perigo.nome());

    let mut frieren = Heroi {
        nome: "Frieren".to_string(),
        vida: 100,
        ataque: 12,
    };
    let mut rng = ChaCha8Rng::seed_from_u64(SEMENTE);

    // `&mut inimigos`: cada `inimigo` é um `&mut Inimigo`, então o duelo altera a vida dele
    for inimigo in &mut inimigos {
        println!("\nFrieren ({} HP) contra {inimigo}", frieren.vida());
        if !duelo(&mut frieren, inimigo, &mut rng) {
            println!("Frieren caiu diante do {}.", inimigo.nome());
            return;
        }
        println!("{} derrotado!", inimigo.nome());
        if inimigo.raridade >= Raridade::Elite {
            println!("Um baú de recompensa aparece.");
        }
    }
    println!("\nTorneio vencido!");
}
