//! MC2 · Rust do zero — Módulo 8 · Aula 1: closures
//! Objetivo: criar funções sem nome que usam as variáveis em volta e passá-las adiante.
//! Rodar:    `cargo run --bin m8_closures`

use std::cmp::Reverse;

// --- 1. Closure: a função que enxerga o ambiente ---------------------------------

fn dobro(x: u32) -> u32 {
    x * 2
}

fn sintaxe() {
    // A mesma conta em três formas: função com nome, closure completa e closure curta
    let dobro_completo = |x: u32| -> u32 { x * 2 };
    let dobro_curto = |x| x * 2; // tipos deduzidos pelo uso (o `5` logo abaixo)
    println!("{} {} {}", dobro(5), dobro_completo(5), dobro_curto(5)); // -> 10 10 10

    // O que a `fn` não faz: usar uma variável de fora (captura, *capture*)
    let bonus = 5;
    let com_bonus = |dano: u32| dano + bonus;
    println!("Dano com bônus: {}", com_bonus(20)); // -> Dano com bônus: 25
}

// --- 2. Captura: emprestar, alterar ou levar ---------------------------------

fn captura() {
    // Só lê: a closure guarda um `&mochila`, e o dono continua podendo ler
    let mochila = vec!["poção", "grimório"];
    let mostrar = || println!("Mochila: {mochila:?}");
    mostrar(); // -> Mochila: ["poção", "grimório"]
    println!("Itens: {}", mochila.len()); // -> Itens: 2

    // Altera: a closure guarda um `&mut golpes` e por isso também precisa de `mut`
    let mut golpes = 0;
    let mut golpear = || golpes += 1;
    golpear();
    golpear();
    println!("Golpes: {golpes}"); // -> Golpes: 2 (o `&mut` acabou no último `golpear()`)

    // Leva: `move` passa a posse para dentro da closure
    let feitico = String::from("Zoltraak");
    let lancar = move || println!("Lançando {feitico}!");
    lancar(); // -> Lançando Zoltraak!
    lancar(); // -> Lançando Zoltraak! (só lê o que guardou: dá para chamar de novo)
}

// --- 3. `Fn`, `FnMut` e `FnOnce`: closures são traits ---------------------------------

// `impl Fn(u32) -> u32` = "qualquer coisa que se chama com um u32, devolve u32 e só lê
// o que capturou". É o `&impl Combatente` do Módulo 7 com outro trait
fn aplicar(dano: u32, efeito: impl Fn(u32) -> u32) -> u32 {
    efeito(dano)
}

// `FnMut`: a closure altera o que capturou, então o parâmetro precisa de `mut`
fn repetir(vezes: u32, mut acao: impl FnMut()) {
    for _ in 0..vezes {
        acao();
    }
}

// `FnOnce`: só dá para chamar uma vez, porque a closure pode entregar o que capturou
fn abrir_bau(abrir: impl FnOnce() -> String) -> String {
    let conteudo = abrir();
    format!("O baú tinha: {conteudo}")
}

fn tres_traits() {
    let fraqueza = 2;
    println!("{}", aplicar(15, |d| d * fraqueza)); // -> 30
    println!("{}", aplicar(15, dobro)); // a função com nome também cumpre o `Fn` -> 30

    let mut mana = 100;
    repetir(3, || mana -= 20);
    println!("Mana: {mana}"); // -> Mana: 40

    let tesouro = String::from("Grimório do Zoltraak");
    // Devolver o `tesouro` move a String para fora da closure: ela vira `FnOnce`
    println!("{}", abrir_bau(|| tesouro)); // -> O baú tinha: Grimório do Zoltraak
}

// --- 4. Guardar e devolver: `impl Fn` e `Box<dyn Fn>` ---------------------------------

// Apelido de tipo (*type alias*): só um nome mais curto, não cria tipo novo
type Buff = Box<dyn Fn(u32) -> u32>;

// Fábrica de closures: cada chamada devolve uma closure com o seu `fator` guardado
fn amplificador(fator: u32) -> impl Fn(u32) -> u32 {
    move |dano| dano * fator // sem `move`: E0373 (o `fator` morre no fim da função)
}

fn guardar() {
    let triplo = amplificador(3);
    println!("{}", triplo(10)); // -> 30

    // Cada closure tem um tipo só dela. Para caberem no mesmo Vec, cada uma vai para uma
    // caixa no heap (`Box`), e o Vec guarda `Box<dyn Fn>`: o `&dyn` do Módulo 7, agora dono
    let buffs: Vec<(&str, Buff)> = vec![
        ("Força", Box::new(|d| d + 10)),
        ("Fúria", Box::new(amplificador(2))),
        ("Maldição", Box::new(|d| d / 4)),
    ];

    let mut dano = 20;
    for (nome, buff) in &buffs {
        dano = buff(dano);
        println!("  {nome}: {dano}"); // -> Força: 30 / Fúria: 60 / Maldição: 15
    }
}

// --- 5. Closures na biblioteca padrão ---------------------------------

fn erro_caro(item: &str) -> String {
    println!("  (montando o erro de {item})");
    format!("sem {item}")
}

struct Membro {
    nome: String,
    nivel: u32,
}

impl Membro {
    fn new(nome: &str, nivel: u32) -> Self {
        Self {
            nome: nome.to_string(),
            nivel,
        }
    }
}

fn na_std() {
    let pocoes: Option<u32> = Some(3);

    // `ok_or` recebe o erro pronto: o `erro_caro` roda mesmo dando certo
    let antes = pocoes.ok_or(erro_caro("poção")); // -> (montando o erro de poção)
    // `ok_or_else` recebe uma closure: o erro só é montado se for None
    let depois = pocoes.ok_or_else(|| erro_caro("poção")); // não imprime nada
    println!("{antes:?} {depois:?}"); // -> Ok(3) Ok(3)

    let mut grupo = vec![
        Membro::new("Stark", 41),
        Membro::new("Frieren", 98),
        Membro::new("Fern", 52),
        Membro::new("Aprendiz", 8),
    ];

    // `sort_by_key`: a closure diz qual campo é a chave de comparação
    grupo.sort_by_key(|m| m.nivel);
    println!("{:?}", grupo.first().map(|m| &m.nome)); // -> Some("Aprendiz")

    // `Reverse` inverte a ordem da chave: do maior para o menor
    grupo.sort_by_key(|m| Reverse(m.nivel));
    // `retain` mantém só quem passa no teste da closure (remove o resto)
    grupo.retain(|m| m.nivel >= 40);
    for m in &grupo {
        println!("  {} (nível {})", m.nome, m.nivel); // -> Frieren, Fern e Stark
    }
}

fn main() {
    sintaxe();
    captura();
    tres_traits();
    guardar();
    na_std();
}
