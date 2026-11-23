//! MC2 · Rust do zero — Módulo 8 · Aula 2: o trait `Iterator`
//! Objetivo: entender o `next()`, criar um iterador próprio e transformar a esteira em resultado.
//! Rodar:    `cargo run --bin m8_iteradores`

// --- 1. A esteira: `next()` ---------------------------------

fn esteira() {
    let drops = ["poção", "maçã", "grimório"];
    let mut esteira = drops.iter(); // `mut`: cada `next()` avança a posição guardada nele

    println!("{:?}", esteira.next()); // -> Some("poção")
    println!("{:?}", esteira.next()); // -> Some("maçã")
    println!("{:?}", esteira.next()); // -> Some("grimório")
    println!("{:?}", esteira.next()); // -> None (acabou)
    println!("Drops: {}", drops.len()); // o `iter()` só emprestou -> Drops: 3
}

// --- 2. Iterador próprio: só o `next()` é obrigatório ---------------------------------

// As ondas de uma masmorra: cada onda traz 3 inimigos a mais que a anterior
struct Ondas {
    atual: u32,
    total: u32,
}

impl Ondas {
    fn new(total: u32) -> Self {
        Self { atual: 0, total }
    }
}

impl Iterator for Ondas {
    // Tipo associado (*associated type*): o tipo do que sai da esteira
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.atual == self.total {
            return None; // passou da última onda
        }
        self.atual += 1;
        Some(self.atual * 3)
    }
}

fn iterador_proprio() {
    // O `for` aceita qualquer Iterator: chama `next()` até vir None
    for inimigos in Ondas::new(3) {
        println!("  Onda com {inimigos} inimigos"); // -> 3, 6 e 9
    }

    // Os métodos padrão do trait vêm de graça, como o `receber` do `Combatente`
    let total: u32 = Ondas::new(5).sum();
    println!("Inimigos em 5 ondas: {total}"); // -> Inimigos em 5 ondas: 45
}

// --- 3. `iter`, `iter_mut` e `into_iter` ---------------------------------

fn tres_jeitos() {
    let mut vidas = vec![30, 0, 45];

    // `iter()`: empresta cada item (`&u32`); é o que o `for v in &vidas` usa
    let soma: u32 = vidas.iter().sum();
    println!("Vida total: {soma}"); // -> Vida total: 75

    // `iter_mut()`: empresta para alterar (`&mut u32`); é o que o `for v in &mut vidas` usa
    for vida in &mut vidas {
        *vida += 10; // cura de gurpo
    }
    println!("{vidas:?}"); // -> [40, 10, 55]

    // `into_iter()`: consome o Vec e entrega cada item com posse; é o `for v in vidas`
    let nomes = vec![String::from("Frieren"), String::from("Fern")];
    let mut esteira = nomes.into_iter();
    let primeira: Option<String> = esteira.next(); // String, não &String
    let segunda = esteira.next();
    println!("{primeira:?} e {segunda:?}"); // -> Some("Frieren") e Some("Fern")
}

// --- 4. Consumidores: a esteira vira um valor ---------------------------------

fn consumidores() {
    let danos = [12, 40, 7, 40, 25];

    // `sum` precisa saber o tipo do resultado: anotação na variável ou turbofish
    let total: u32 = danos.iter().sum();
    println!("Dano total: {total}"); // -> Dano total: 124
    println!("Maior: {:?}", danos.iter().max()); // -> Maior: Some(40)
    println!("Menor: {:?}", danos.iter().min()); // -> Menor: Some(7)

    let vazio: Vec<u32> = Vec::new();
    println!("Maior do vazio: {:?}", vazio.iter().max()); // -> Maior do vazio: None

    println!("Ondas: {}", Ondas::new(4).count()); // -> Ondas: 4
    println!("Última: {:?}", Ondas::new(4).last()); // -> Última: Some(12)
    println!("Letras: {}", "Zoltraak".chars().count()); // -> Letras: 8
}

// --- 5. Consumidores com closure ---------------------------------

struct Inimigo {
    nome: String,
    ataque: u32,
    vida: u32,
}

impl Inimigo {
    fn new(nome: &str, ataque: u32, vida: u32) -> Self {
        Self {
            nome: nome.to_string(),
            ataque,
            vida,
        }
    }
}

fn com_closure() {
    let horda = [
        Inimigo::new("Slime", 3, 20),
        Inimigo::new("Golem", 6, 40),
        Inimigo::new("Mímico", 6, 25),
        Inimigo::new("Dragão", 10, 70),
    ];

    // `find`: o primeiro que passa no teste, ou None
    if let Some(golem) = horda.iter().find(|i| i.nome == "Golem") {
        println!("Achei o {} ({} HP)", golem.nome, golem.vida); // -> Achei o Golem (40 HP)
    }
    let danos = [12, 40, 7];
    println!("{:?}", danos.iter().find(|&&d| d > 30)); // -> Some(40)

    // `position`: o índice do primeiro que passa
    println!("{:?}", horda.iter().position(|i| i.nome == "Mímico")); // -> Some(2)

    // `any`: algum passa? `all`: todos passam?
    let tem_chefe = horda.iter().any(|i| i.ataque >= 10);
    let todos_vivos = horda.iter().all(|i| i.vida > 0);
    println!("Chefe? {tem_chefe}. Todos vivos? {todos_vivos}"); // -> true e true

    // `max_by_key`: o `mais_perigoso` do Módulo 7 numa linha, para qualquer tamanho de lista
    if let Some(perigo) = horda.iter().max_by_key(|i| i.ataque) {
        println!("Mais perigoso: {}", perigo.nome); // -> Mais perigoso: Dragão
    }
    // Empate: `max_by_key` devolve o último; `min_by_key`, o primeiro
    let sem_chefe = &horda[..3];
    let max = sem_chefe.iter().max_by_key(|i| i.ataque).map(|i| &i.nome);
    let min = sem_chefe.iter().min_by_key(|i| i.vida).map(|i| &i.nome);
    println!("{max:?} {min:?}"); // -> Some("Mímico") Some("Slime")

    // `fold`: o consumidor geral. Começa com 0 e junta cada item ao acumulado
    let vida_total = horda.iter().fold(0, |soma, i| soma + i.vida);
    println!("Vida da horda: {vida_total}"); // -> Vida da horda: 155
}

fn main() {
    esteira();
    iterador_proprio();
    tres_jeitos();
    consumidores();
    com_closure();
}
