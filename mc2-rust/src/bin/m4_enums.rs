//! MC2 · Rust do zero - Módulo 4 · Aula 1: enums e `match`
//! Objetivo: modelar estados e ações de um personagem com `enum` e tratar cada caso com `match`.
//! Rodar: cargo run --bin m4_enums

// --- 1. Enum simples: o estado do personagem ---------------------------------

// Um `enum` (enumeration, enumeração) lista TODAS as formas que um valor pode ter.
// Cada forma é uma *variant* (variante), e um `Estado` é sempre exatamente uma delas.
#[derive(Debug)]
enum Estado {
    Parado,
    Andando,
    Correndo,
    Atordoado,
}

// --- 2. `match` e `impl` em enum ---------------------------------

// o `impl` funciona igual ao das structs (Módulo 3): `&self` lê, `&mut self` altera.
impl Estado {
    // `match` testa os padrões (patterns) de cima para baixo e roda o braço (arm) do primeiro
    // que casar. É uma expressão: o valor do braço escolhido vira o retorno da função.
    fn velocidade(&self) -> u32 {
        match self {
            Estado::Parado => 0,
            Estado::Andando => 3,
            Estado::Correndo => 8,
            Estado::Atordoado => 0,
        }
    }
    // ❌ E0004: non-exhaustive patterns: `&Estado::Atordoado` not covered
    // (apagar o braço `Estado::Atordoado => 0` acima). O match tem que cobrir TODAS as
    // variantes; esquecer uma não é aviso, é erro de compilação.

    // `_` (curinga, catch-all) casa com qualquer coisa. Prático, mas desliga a proteção:
    // uma variante nova (ex.: `Congelado`) cairia aqui calada, sem erro.
    // Os literais são `&'static str` (gravados no executável, Módulo 2), por isso podem
    // sair por um `-> &str`.
    fn sprite(&self) -> &str {
        match self {
            Estado::Correndo => "frieren_corre.png",
            Estado::Atordoado => "frieren_tonta.png",
            _ => "frieren_base.png",
        }
    }
}

// --- 3. Enum com dados: as ações ---------------------------------

// Cada variante pode carregar dados próprios, de formatos diferentes.
#[derive(Debug)]
enum Acao {
    Esperar,                  // sem dados (como no C)
    Atacar(u32),              // um valor, como a tuple struct `Dano(u32)` do Módulo 3
    Mover { x: i32, y: i32 }, // campos com nome, como uma struct
    Falar(String),            // dona de um texto no heap (o drop da Acao libera junto)
}

// --- 4. `match` com dados: desestruturação, `|`, guarda e faixas ---------------------------------

impl Acao {
    fn descrever(&self) -> String {
        match self {
            Acao::Esperar => String::from("espera o próximo turno"),
            // `Atacar(dano)` casa E tira o valor de dentro: `dano` é uma variável nova
            // (binding), que só existe neste braço
            Acao::Atacar(dano) => format!("ataca causando {dano} de dano"),
            Acao::Mover { x, y } => format!("anda até ({x}, {y})"),
            Acao::Falar(fala) => format!("diz \"{fala}\""),
        }
    }

    fn custo_energia(&self) -> u32 {
        match self {
            // `|` junta padrões num braço só ("ou")
            Acao::Esperar | Acao::Falar(_) => 0,
            // guarda (match guard): um `if` a mais no padrão. Como o match é em `&Acao`,
            // `dano` aqui é `&u32` (match ergonomics), por isso o `*` do Módulo 2
            Acao::Atacar(dano) if *dano >= 50 => 20,
            Acao::Atacar(_) => 10,
            // `unsigned_abs()` = valor absoluto já como `u32` (nada de `as`)
            Acao::Mover { x, y } => x.unsigned_abs() + y.unsigned_abs(),
        }
    }

    // Padrões se encaixam uns dentro dos outros: uma faixa (range) dentro da variante.
    fn intensidade(&self) -> &str {
        match self {
            Acao::Atacar(0) => "errou",
            Acao::Atacar(1..=20) => "fraco",
            Acao::Atacar(21..=49) => "médio",
            Acao::Atacar(_) => "crítico",
            _ => "-",
        }
    }
}

// --- 5. `match` em dupla: estado + ação ---------------------------------

// Casar uma tupla testa as duas coisas de uma vez: o começo de uma máquina de estados.
fn resolver_turno(estado: &Estado, acao: &Acao) -> String {
    match (estado, acao) {
        (Estado::Atordoado, _) => String::from("está atordoada e perde a vez"),
        (Estado::Correndo, Acao::Atacar(dano)) => {
            format!("investe correndo: {} de dano", dano * 2)
        }
        (_, acao) => acao.descrever(),
    }
}

// --- 6. Comparando variantes: `matches!` ---------------------------------

// Pode haver vários blocos `impl` para o mesmo tipo (Módulo 3).
impl Estado {
    // `matches!` é uma macro que devolve `bool`: "este valor casa com este padrão?"
    fn pode_agir(&self) -> bool {
        !matches!(self, Estado::Atordoado)
    }
}
// ❌ E0369: binary operation `!=` cannot be applied to type `&Estado`
// (trocar o corpo por `self != &Estado::Atordoado`). Em C o enum é um `int` e o `==` vem
// de graça; aquii é um tipo próprio, e o `==` exige `#[derive](PartialEq)]` (Módulo 7).

// --- 7. Testando tudo ---------------------------------

fn main() {
    // 1. Enum simples: a variante se escreve `Tipo::Variante`
    let estado = Estado::Correndo;
    println!("{estado:?}"); // -> Correndo

    // 2 e 6. match, impl e matches!, para cada estado
    for e in [
        Estado::Parado,
        Estado::Andando,
        Estado::Correndo,
        Estado::Atordoado,
    ] {
        println!(
            "{e:?}: velocidade {}, {}, age? {}",
            e.velocidade(),
            e.sprite(),
            e.pode_agir()
        );
    }
    // -> Parado: velocidade 0, frieren_base.png, age? true
    // -> Andando: velocidade 3, frieren_base.png, age? true
    // -> Correndo: velocidade 8, frieren_corre.png, age? true
    // -> Atordoado: velocidade 0, frieren_tonta.png, age? false

    // 3 e 4. Enum com dados + match com dados
    let acoes = [
        Acao::Esperar,
        Acao::Atacar(12),
        Acao::Atacar(60),
        Acao::Mover { x: 3, y: -4 },
        Acao::Falar(String::from("Zoltraak!")),
    ];
    println!("{:?}", acoes[3]); // -> Mover { x: 3, y: -4}

    for acao in &acoes {
        println!(
            "Frieren {} [{}, custo {}]",
            acao.descrever(),
            acao.intensidade(),
            acao.custo_energia()
        );
    }
    // -> Frieren espera o próximo turno [-, custo 0]
    // -> Frieren ataca causando 12 de dano [fraco, custo 10]
    // -> Frieren ataca causando 60 de dano [crítico, custo 20]
    // -> Frieren anda até (3, -4) [-, custo 7]
    // -> Frieren diz "Zoltraak!" [-, custo 0]

    // 5. match em dupla
    println!("{}", resolver_turno(&Estado::Atordoado, &acoes[1]));
    // -> está atordoada e perde a vez
    println!("{}", resolver_turno(&estado, &acoes[1])); // -> investe correndo: 24 de dano
    println!("{}", resolver_turno(&Estado::Parado, &acoes[3])); // -> anda até (3, -4)

    // 6. O `matches!` também aceita guarda
    let forte = matches!(acoes[2], Acao::Atacar(dano) if dano >= 50);
    println!("golpe forte? {forte}"); // -> golpe forte? true
}
