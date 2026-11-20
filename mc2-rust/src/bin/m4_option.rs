//! MC2 · Rust do zero - Módulo 4 · Aula 2: `Option`, `if let` e `let ... else`
//! Objetivo: representar "pode não ter valor" sem `null` e tratar os dois casos.
//! Rodar:  cargo run --bin m4_option

// ---1. `Option` como retorno: o fim do panic do `maior_nota` ------------------

// `Option<T>` é um enum comum da biblioteca padrão: `enum Option<T> { None, Some(T) }`.
// `<T>` = "de qualquer tipo" (genéricos, Módulo 7). Já vem importado (prelude), por isso
// `Some` e `None` aparecem sem `Option::` na frente.
fn maior_nota(notas: &[f64]) -> Option<f64> {
    // `.first()` já devolve `Option<&f64>`: `Some(&primeira)`, ou `None` se a fatia estiver vazia
    let mut maior = match notas.first() {
        Some(&n) => n,       // o `&n` no padrão tira o `f64` de dentro da referência
        None => return None, // fatia vazia: não existe maior nota, sai mais cedo
    };
    for &n in notas {
        if n > maior {
            maior = n;
        }
    }
    Some(maior)
}
// ❌ E0308: mismatched types: expected `f64`, found `Option<f64>`
// (escrever `let m: f64 = maior_nota(&nota);`). O `Option` não vira `f64` sozinho:
// o compilador obriga a decidir o que acontece no `None`.

// `match` em `Option`: os dois casos, como em qualquer enum
fn anunciar(nota: Option<f64>) -> String {
    match nota {
        Some(n) => format!("maior nota: {n:.1}"),
        None => String::from("sem notas ainda"),
    }
}

// --- 2. `Option` em campo de struct: a arma equipada ------------------

#[derive(Debug)]
struct Item {
    nome: String,
    poder: u32,
}

struct Personagem {
    nome: String,
    arma: Option<Item>, // pode estar de mãos vazias: o "talvez" fica no tipo, não num null
}

impl Personagem {
    fn new(nome: &str) -> Self {
        Self {
            nome: nome.to_string(),
            arma: None,
        }
    }

    fn dano(&self) -> u32 {
        // `&self.arma`: empresta o Option. Sem o `&`, o match tentaria MOVER a arma para fora
        // de um `self` que só está emprestado
        match &self.arma {
            Some(arma) => 5 + arma.poder,
            None => 5, // soco
        }
    }

    // `.replace(nova)` põe a nova arma e devolve a antiga (`Some`), ou `None` se não havia
    fn equipar(&mut self, nova: Item) -> Option<Item> {
        self.arma.replace(nova)
    }

    // `.take()` tira o valor e deixa `None` no lugar: move sem `.clone()`, mesmo com `&mut`
    fn desarmar(&mut self) -> Option<Item> {
        self.arma.take()
    }
}

// --- 3. `if let`: quando só um caso importa ------------------

impl Personagem {
    fn mostrar_arma(&self) {
        // `if let PADRÃO = valor { ... }` = match de um braço só (o `else` é opcional)
        if let Some(arma) = &self.arma {
            println!("{} empunha {} (+{})", self.nome, arma.nome, arma.poder);
        } else {
            println!("{} está de mãos vazias", self.nome);
        }
    }
}

// --- 4. `let ... else`: tira o valor ou sai ------------------

// `.get(i)` devolve `Option<&Item>`; o `mochila[i]` daria panic fora do tamanho (Módulo 1)
fn ver_slot(mochila: &[Item], slot: usize) -> String {
    // Se casar, `item` fica disponível no resto da função, já sem o Option em volta.
    // Se não casar, o `else` roda e PRECISA sair (`return`, `break`, `panic!`...).
    let Some(item) = mochila.get(slot) else {
        return format!("slot {slot}: vazio");
    };
    format!("slot {slot}: {} (+{})", item.nome, item.poder)
}

// --- 5. Testando tudo + atalhos do `Option` ------------------

fn main() {
    // 1. Option como retorno + match
    let notas = [7.5, 9.0, 6.0];
    println!("{:?}", maior_nota(&notas)); // -> Some(9.0)
    println!("{:?}", maior_nota(&[])); // -> None (no Módulo 2, isto era panic)
    println!("{}", anunciar(maior_nota(&notas))); // -> maior nota: 9.0
    println!("{}", anunciar(maior_nota(&[]))); // -> sem notas ainda

    // 2 e 3. Option em struct + if let
    let mut frieren = Personagem::new("Frieren");
    frieren.mostrar_arma(); // -> Frieren está de mãos vazias
    println!("dano: {}", frieren.dano()); // -> dano: 5

    let antiga = frieren.equipar(Item {
        nome: String::from("Cajado"),
        poder: 30,
    });
    println!("{antiga:?}"); // -> None (não havia arma antes)
    frieren.mostrar_arma(); // -> Frieren empunha Cajado (+30)
    println!("dano: {}", frieren.dano()); // -> dano: 35

    // `if let` direto no retorno: só faz algo se a troca devolveu uma arma
    let espada = Item {
        nome: String::from("Espada do Herói"),
        poder: 50,
    };
    if let Some(velha) = frieren.equipar(espada) {
        println!("guardou {} na mochila", velha.nome); // -> guardou Cajado na mochila
    }
    frieren.desarmar(); // devolve Some(Espada do Herói); aqui o retorno é descartado
    frieren.mostrar_arma(); // -> Frieren está de mãos vazias

    // 4. let ... else
    let mochila = [
        Item {
            nome: String::from("Poção"),
            poder: 20,
        },
        Item {
            nome: String::from("Grimório"),
            poder: 80,
        },
    ];
    println!("{}", ver_slot(&mochila, 1)); // -> slot 1: Grimório (+80)
    println!("{}", ver_slot(&mochila, 5)); // -> slot 5: vazio

    // 5. Atalhos: o `??` e o `!` do TypeScript
    println!("{}", maior_nota(&[]).unwrap_or(0.0)); // -> 0 (valor padrão se for None)
    println!("{}", maior_nota(&notas).is_some()); // -> true
    // `.unwrap()` tira o valor ou dá panic no None: usar só quando o None for impossível
    println!("{}", maior_nota(&notas).unwrap()); // -> 9
    // maior_nota(&[]).unwrap(); // panic: called `Option::unwrap()` on a `None` value
}
