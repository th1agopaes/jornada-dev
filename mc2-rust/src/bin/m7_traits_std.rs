//! MC2 · Rust do zero — Módulo 7 · Aula 3: traits da biblioteca padrão
//! Objetivo: `derive` para `==`, ordem e hash; `Display` para texto; `From` para o `?`.
//! Rodar:    `cargo run --bin m7_traits_std`

use std::collections::HashMap;
use std::fmt;
use std::num::ParseIntError;

// --- 1. `#[derive]`: o compilador escreve o `impl` ---------------------------------

// Cada nome no `derive` é um trait que o compilador implementa sozinho, variante a
// variante (ou campo a campo). A ordem das variantes vira a ordem do `<` e do `sort()`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Raridade {
    Comum,
    Rara,
    Lendaria,
}

// `String` não é `Copy`, então o Item só pode ser `Clone` (a cópia explícita do Módulo 2)
#[derive(Debug, Clone, PartialEq)]
struct Item {
    nome: String,
    raridade: Raridade,
}

fn derive() {
    use Raridade::{Comum, Lendaria, Rara}; // atalho para as variantes (o `use` do Módulo 6)

    let achada = Rara;
    let copia = achada; // Copy: `achada` continua valendo (sem o Copy, seria move e E0382)
    println!("{achada:?} == {copia:?}? {}", achada == copia); // -> Rara == Rara? true
    println!("Lendária > Comum? {}", Lendaria > Comum); // -> Lendária > Comum? true

    let mut drops = vec![Lendaria, Comum, Rara, Comum];
    drops.sort(); // Ord
    println!("{drops:?}"); // -> [Comum, Comum, Rara, Lendaria]

    // Hash + Eq: a Raridade pode ser chave de HashMap (o `entry`)
    let mut contagem = HashMap::new();
    for r in &drops {
        *contagem.entry(*r).or_insert(0) += 1;
    }
    println!("Comuns: {:?}", contagem.get(&Comum)); // -> Comuns: Some(2)

    let espada = Item {
        nome: "espada".to_string(),
        raridade: Rara,
    };
    let reserva = espada.clone(); // Clone: copia o heap também
    println!("Iguais? {}", espada == reserva); // -> Iguais? true
}

// --- 2. `Display`: o texto para quem joga ---------------------------------

// Debug (`{:?}`) é para quem programa e vem do `derive`. Display (`{}`) é para quem joga
// e não tem `derive`: você escolhe o texto
impl fmt::Display for Raridade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let texto = match self {
            Raridade::Comum => "comum",
            Raridade::Rara => "rara",
            Raridade::Lendaria => "lendária",
        };
        write!(f, "{texto}") // `write!` = `format!` que escreve no Formatter
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.nome, self.raridade) // o `{}` da raridade usa o Display dela
    }
}

fn display() {
    let cajado = Item {
        nome: "cajado".to_string(),
        raridade: Raridade::Lendaria,
    };
    println!("{cajado}"); // -> cajado [lendária]
    println!("{cajado:?}"); // -> Item { nome: "cajado", raridade: Lendaria }

    let texto = cajado.to_string(); // `to_string()` vem de graça com o Display
    println!("{} letras", texto.chars().count()); // -> 17 letras
}

// --- 3. Erro próprio com `Display` ---------------------------------

#[derive(Debug)]
enum ErroJogo {
    ComandoDesconhecido(String),
    QtdInvalida(ParseIntError), // guarda o erro original do `parse`
    QtdZero,
}

// No lugar do `mensagem()` dos Módulos 5 e 6: com Display, o erro entra direto no `{}`
impl fmt::Display for ErroJogo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroJogo::ComandoDesconhecido(comando) => write!(f, "comando desconhecido: {comando}"),
            ErroJogo::QtdInvalida(erro) => write!(f, "quantidade inválida ({erro})"),
            ErroJogo::QtdZero => write!(f, "a quantidade precisa ser maior que 0"),
        }
    }
}

// `Error` é o trait padrão dos erros. Ele exige Debug + Display (supertraits) e todos os
// métodos dele têm padrão, então o `impl` fica vazio
impl std::error::Error for ErroJogo {}

fn erro_proprio() {
    let erros = [
        ErroJogo::ComandoDesconhecido("dançar".to_string()),
        ErroJogo::QtdZero,
    ];
    for erro in &erros {
        println!("Erro: {erro}"); // -> Erro: comando desconhecido: dançar / Erro: a quantidade...
    }
    println!("{:?}", erros[1]); // ->QtdZero
}

// --- 4. `From`: o `?` converte o erro sozinho ---------------------------------

// "Como fazer um ErroJogo a partir de um ParseIntError". O `?` chama este `from`
// quando o erro que chegou não é o tipo de erro que a função devolve
impl From<ParseIntError> for ErroJogo {
    fn from(erro: ParseIntError) -> Self {
        ErroJogo::QtdInvalida(erro)
    }
}

fn ler_qtd(texto: &str) -> Result<u32, ErroJogo> {
    // Turbofish: o tipo do erro (ParseIntError) fica certo antes de o compilador olhar o `?`
    let qtd = texto.parse::<u32>()?; // Err(ParseIntError) -> ErroJogo::from(...) -> return Err
    if qtd == 0 {
        return Err(ErroJogo::QtdZero); // a invariante do Módulo 5: nenhum item com 0
    }
    Ok(qtd)
}

fn conversao() {
    for entrada in ["3", "três", "0", "99999999999"] {
        match ler_qtd(entrada) {
            Ok(qtd) => println!("{entrada}: pegou {qtd}"),
            Err(erro) => println!("{entrada}: {erro}"),
        }
    }

    // `String::from` sempre foi este trait: `impl From<&str> for String`.
    // E quem tem `From` ganha o `into()`, o mesmo caminho pelo outro lado
    let nome: String = "Frieren".into();
    println!("{nome}"); // -> Frieren
}

fn main() {
    derive();
    display();
    erro_proprio();
    conversao();
}
