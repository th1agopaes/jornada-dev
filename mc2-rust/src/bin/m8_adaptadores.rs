//! MC2 · Rust do zero — Módulo 8 · Aula 3: adaptadores e `collect`
//! Objetivo: montar esteiras com `map`, `filter` e companhia e juntar o resultado com `collect`.
//! Rodar:    `cargo run --bin m8_adaptadores`

use std::collections::HashMap;

// --- 1. Adaptadores são preguiçosos ---------------------------------

fn preguica() {
    let danos = [10, 20, 30];

    // `map` e `filter` só montam a esteira; quem puxa os itens é o consumidor (`sum`).
    // Os `println!` mostram a ordem: cada item passa pela esteira inteira antes do próximo
    let total: u32 = danos
        .iter()
        .map(|d| {
            println!("  dobrando {d}");
            d * 2
        })
        .filter(|d| {
            println!("  conferindo {d}");
            *d > 25
        })
        .sum();
    println!("Total acima de 25: {total}"); // -> Total acima de 25: 100
}

// --- 2. `map`, `filter` e `filter_map` ---------------------------------

fn transformar() {
    let horda = [("Slime", 20), ("Golem", 40), ("Mímico", 25), ("Dragão", 70)];

    // `filter` deixa passar quem a closure aprova; `map` transforma cada item que passou
    let resistentes = horda
        .iter()
        .filter(|(_, vida)| *vida >= 30)
        .map(|(nome, _)| nome);
    for nome in resistentes {
        println!("  Resistente: {nome}"); // -> Golem e Dragão
    }

    // `filter_map`: a closure devolve Option; Some(x) segue como x, None é descartado
    let entradas = ["3", "muitas", "5", "-1"];
    let validas: u32 = entradas.iter().filter_map(|t| t.parse::<u32>().ok()).sum();
    println!("Poções válidas: {validas}"); // -> Poções válidas: 8
}

// --- 3. `collect`: da esteira para uma coleção ---------------------------------

fn juntar() {
    let nomes = ["frieren", "fern", "stark"];

    // Vec: o tipo de destino vem da anotação
    let gritos: Vec<String> = nomes.iter().map(|n| n.to_uppercase()).collect();
    println!("{gritos:?}"); // -> ["FRIEREN", "FERN", "STARK"]

    // String: junta os `char` num texto só
    let iniciais: String = nomes.iter().filter_map(|n| n.chars().next()).collect();
    println!("{iniciais}"); // -> ffs

    // HashMap: cada item precisa ser uma tupla (chave, valor)
    let loja = "poção=30\nmaçã=5\ngrimório=300";
    let precos: HashMap<&str, u32> = loja
        .lines()
        .filter_map(|linha| linha.split_once('='))
        .map(|(item, preco)| (item, preco.parse().unwrap_or(0)))
        .collect();
    let grimorio = precos.get("grimório");
    println!("{} itens; grimório: {grimorio:?}", precos.len()); // -> 3 itens; grimório: Some(300)

    // Result: Ok(Vec) se todos derem Ok; no primeiro Err, para e devolve esse Err.
    // O `_` deixa o compilador deduzir o tipo do erro (ParseIntError)
    let boas: Result<Vec<u32>, _> = ["3", "5"].iter().map(|t| t.parse::<u32>()).collect();
    let ruins: Result<Vec<u32>, _> = ["3", "x"].iter().map(|t| t.parse::<u32>()).collect();
    println!("{boas:?}"); // -> Ok([3, 5])
    println!("{}", ruins.is_err()); // -> true
}

// --- 4. `zip`, `enumerate`, `take`, `skip` e `rev` ---------------------------------

fn mais_adaptadores() {
    let ranking = ["Frieren", "Fern", "Stark", "Sein"];
    let pontos = [980, 720, 655, 300];

    // `zip` anda com duas esteiras juntas, em pares; `enumerate` numera; `take(3)` para no 3º
    for (i, (nome, pts)) in ranking.iter().zip(pontos).enumerate().take(3) {
        println!("  {}º {nome} ({pts})", i + 1); // -> 1º Frieren (980) ...
    }
    // `skip(3)` pula os 3 primeiros; `copied` troca cada `&&str` por uma cópia do `&str`
    let fora: Vec<&str> = ranking.iter().skip(3).copied().collect();
    println!("Fora do pódio: {fora:?}"); // -> Fora do pódio: ["Sein"]

    // `rev` anda de trás para frente; `join` junta um Vec<String> com um separador
    let contagem: Vec<String> = (1..=3).rev().map(|n| n.to_string()).collect();
    println!("{}... Zoltraak!", contagem.join("... ")); // -> 3... 2... 1... Zoltraak!

    // `1..` não tem fim: só funciona porque o `take` para de puxar depois de 5
    let xp: Vec<u32> = (1..).map(|nivel| nivel * nivel * 100).take(5).collect();
    println!("XP por nível: {xp:?}"); // -> [100, 400, 900, 1600, 2500]

    // `take_while`: puxa enquanto a closure aprovar e para no primeiro "não"
    let ate_mil: Vec<u32> = xp.iter().copied().take_while(|&x| x <= 1000).collect();
    println!("Até 1000 XP: {ate_mil:?}"); // -> Até 1000 XP: [100, 400, 900]
}

// --- 5. O `search` do minigrep ---------------------------------

// O retorno empresta do `texto` (o `depois_de` do Módulo 7): só ele ganha o `'a`
fn buscar<'a>(consulta: &str, texto: &'a str) -> Vec<&'a str> {
    texto
        .lines()
        .filter(|linha| linha.contains(consulta))
        .collect()
}

// Sem diferenciar maiúsculas: a closure captura a `consulta` já em minúsculas
fn buscar_sem_caixa<'a>(consulta: &str, texto: &'a str) -> Vec<&'a str> {
    let consulta = consulta.to_lowercase();
    texto
        .lines()
        .filter(|linha| linha.to_lowercase().contains(&consulta))
        .collect()
}

fn minigrep() {
    let diario = "Dia 1: Frieren achou um grimório.\n\
        Dia 2: Fern treinou o Zoltraak.\n\
        Dia 3: o Mímico mordeu a Frieren.\n\
        Dia 4: Stark enfrentou um dragão.";

    for linha in buscar("Frieren", diario) {
        println!("  {linha}"); // -> Dia 1 e Dia 3
    }
    println!("{:?}", buscar("mímico", diario)); // -> [] (o diário escreve "Mímico")
    println!("{:?}", buscar_sem_caixa("MÍMICO", diario));
    // -> ["Dia 3: o Mímico mordeu a Frieren."]
}

fn main() {
    preguica();
    transformar();
    juntar();
    mais_adaptadores();
    minigrep();
}
