//! MC2 · Rust do zero — Módulo 7 · Aula 2: genéricos
//! Objetivo: escrever uma função ou struct uma vez só, para vários tipos, com limites de trait.
//! Rodar:    `cargo run --bin m7_genericos`

use std::fmt::Debug; // trait de impressão com `{:?}`, usado como limite (seção 3)

use rand::rngs::ChaCha8Rng;
use rand::{Rng, RngExt, SeedableRng}; // `Rng` = trait-base dos geradores (seção 4)

// --- 1. Função genérica e limite de trait ---------------------------------

// Sem genéricos, seriam `maior_dano(&[u32])`, `maior_chance(&[f64])`, `maior_nome(&[&str])`...
// com o mesmo corpo copiado. `<T>` é um parâmetro de tipo: o tipo entra na chamada.
// `T: PartialOrd` = "qualquer T que saiba se comparar com `<` e `>`"
fn maior<T: PartialOrd>(lista: &[T]) -> Option<&T> {
    let mut maior = lista.first()?; // lista vazia: devolve None aqui mesmo
    for item in lista {
        if item > maior {
            maior = item;
        }
    }
    Some(maior)
}

fn funcao_generica() {
    println!("{:?}", maior(&[12, 45, 30])); // -> Some(45)
    println!("{:?}", maior(&["pena", "maçã", "grimório"])); // -> Some("pena")
    println!("{:?}", maior::<u32>(&[])); // -> None (vazia: o turbofish diz qual é o T)
}

// --- 2. Struct genérica: `Bau<T>` ---------------------------------

// Um baú que guarda qualquer coisa: o `T` vira o tipo do campo `conteudo`
struct Bau<T> {
    local: String,
    conteudo: T,
}

// `impl<T>`: métodos para baús de qualquer tipo (o `<T>` depois do `impl` declara o T)
impl<T> Bau<T> {
    fn new(local: &str, conteudo: T) -> Self {
        Self {
            local: local.to_string(),
            conteudo,
        }
    }

    fn local(&self) -> &str {
        &self.local
    }

    // Consome o baú e entrega o conteúdo com posse, seja qual for o tipo
    fn abrir(self) -> T {
        self.conteudo
    }
}

// Sem `<T>`: métodos só para baús de moedas
impl Bau<u32> {
    fn dobrar(&mut self) {
        self.conteudo *= 2;
    }
}

fn struct_generica() {
    let mut ouro: Bau<u32> = Bau::new("sala 1", 150);
    ouro.dobrar(); // só existe em Bau<u32>; num Bau<String> seria E0599
    println!("Baú da {}:", ouro.local()); // -> Baú da sala 1:
    println!("  {} de ouro", ouro.abrir()); // -> 300 de ouro

    let livro = Bau::new("sala 3", String::from("grimório do Zoltraak")); // Bau<String>
    println!("  {}", livro.abrir()); // -> grimório do Zoltraak
}

// --- 3. Vários limites e `where` ---------------------------------

// `T: PartialOrd + Debug`: T precisa cumprir os dois contratos. Com muitos limites,
// o `where` tira a lista da frente da assinatura
fn anunciar_maior<T>(rotulo: &str, lista: &[T])
where
    T: PartialOrd + Debug, // PartialOrd para chamar o `maior`; Debug para o `{:?}`
{
    match maior(lista) {
        Some(item) => println!("{rotulo}: {item:?}"),
        None => println!("{rotulo}: nada"),
    }
}

// Com `<T>`, os dois parâmetros são do MESMO tipo. Com `&impl PartialEq` em cada um,
// poderiam ser tipos diferentes (como o `golpear` da Aula 1)
fn empatou<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

fn limites() {
    anunciar_maior("Maior chance", &[0.25, 0.5, 0.1]); // -> Maior chance: 0.5
    anunciar_maior("Maior nível", &[7_u8, 3, 9]); // -> Maior nível: 9
    println!("Empate? {}", empatou(&45, &45)); // -> Empate? true
}

// --- 4. Gerador genérico: `&mut impl Rng` ---------------------------------

// Aceita qualquer gerador: o da thread no jogo, um com semente para conferir a saída
fn sortear(rng: &mut impl Rng) -> &'static str {
    let rolagem: u32 = rng.random_range(1..=100); // `random_range` vem do `RngExt`
    match rolagem {
        1..=60 => "poção",
        61..=90 => "maçã",
        _ => "grimório",
    }
}

fn geradores() {
    let mut rng = rand::rng(); // ThreadRng: semente do sistema operacional
    println!("Drop do dia: {}", sortear(&mut rng)); // -> muda a cada execução

    let mut semente = ChaCha8Rng::seed_from_u64(42); // a mesma função, outro tipo de gerador
    print!("Com semente:");
    for _ in 0..5 {
        print!(" {}", sortear(&mut semente));
    }
    println!(); // -> Com semente: poção maçã poção grimório maçã
}

// --- 5. Tipos diferentes na mesma lista: `dyn` ---------------------------------

trait Interagivel {
    fn interagir(&self) -> String;
}

struct Porta {
    trancada: bool,
}

struct Placa(String);

impl Interagivel for Porta {
    fn interagir(&self) -> String {
        if self.trancada {
            "A porta está trancada.".to_string()
        } else {
            "A porta range e abre.".to_string()
        }
    }
}

impl Interagivel for Placa {
    fn interagir(&self) -> String {
        format!("A placa diz: \"{}\"", self.0)
    }
}

// Um trait pode ser implementado só para uma versão do genérico
impl Interagivel for Bau<u32> {
    fn interagir(&self) -> String {
        format!("Baú da {} com {} de ouro.", self.local, self.conteudo)
    }
}

fn sala() {
    let porta = Porta { trancada: true };
    let placa = Placa("Cuidado com o Mímico".to_string());
    let bau: Bau<u32> = Bau::new("sala 2", 80);

    // `&dyn Interagivel` = "referência para algum tipo que implementa Interagivel,
    // descoberto só ao rodar". Assim, tipos diferentes cabem no mesmo Vec
    let objetos: Vec<&dyn Interagivel> = vec![&porta, &placa, &bau];
    for objeto in objetos {
        println!("{}", objeto.interagir());
    }
}

fn main() {
    funcao_generica();
    struct_generica();
    limites();
    geradores();
    sala();
}
