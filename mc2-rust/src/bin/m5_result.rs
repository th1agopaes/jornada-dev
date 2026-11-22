//! MC2 · Rust do zero — Módulo 5 · Aula 3: `Result<T, E>` e o operador `?`
//! Objetivo: tratar falhas esperadas como valor, repassar com `?` e ler comandos do teclado.
//! Rodar:    `cargo run --bin m5_result`

use std::io;
use std::num::ParseIntError;

// --- 1. Result: deu certo ou deu errado ---------------------------------

fn converter() {
    // `parse` converte texto e pode falhar: devolve Result<u32, ParseIntError>
    // O `::<u32>` (turbofish) diz para qual tipo converter
    let certo = "42".parse::<u32>();
    let errado = "abc".parse::<u32>();
    println!("{certo:?}"); // -> Ok(42)
    println!("{errado:?}"); // -> Err(ParseIntError { kind: InvalidDigit })

    // Result é um enum como o Option: um braço do `match` para cada variante
    match "-5".parse::<u32>() {
        Ok(n) => println!("Dano: {n}"),
        Err(e) => println!("Inválido: {e}"), // -> Inválido: invalid digit found in string
    }
}

// --- 2. Os atalhos do Option valem aqui ---------------------------------

fn atalhos() {
    // `if let` quando só o Ok interessa
    if let Ok(vida) = "150".parse::<u32>() {
        println!("Vida: {vida}"); // -> Vida: 150
    }

    // `unwrap_or` troca qualquer Err por um valor padrão
    let nivel = "dez".parse::<u32>().unwrap_or(1);
    println!("Nível: {nivel}"); // -> Nível: 1

    // `is_ok` e `is_err` só respondem sim ou não
    println!("{}", "300".parse::<u8>().is_err()); // -> true (o u8 vai só ate 255)

    // `let ... else`: a cláusula de guarda do Módulo 4
    let Ok(ouro) = "1000".parse::<u32>() else {
        println!("Ouro inválido");
        return;
    };
    println!("Ouro: {ouro}"); // -> Ouro: 1000

    // Com a anotação no `let`, o `parse` dispensa o turbofish.
    // `expect``dá panic no Err: só quando falhar é bug (aqui o texto é fixo e válido)
    let maximo: u32 = "999".parse().expect("constante do jogo");
    println!("Máximo: {maximo}"); // -> Máximo: 999
}

// --- 3. O operador `?`: tira o valor ou sai ---------------------------------

fn somar_textos(a: &str, b: &str) -> Result<u32, ParseIntError> {
    // `?` = "se for Ok, tira o valor; se for Err, devolve o Err agora". Por extenso:
    // let x = match a.parse::<u32>() { Ok(n) => n, Err(e) => return Err(e) };
    let x = a.parse::<u32>()?;
    let y = b.parse::<u32>()?;
    Ok(x + y)
}

// O `?` também funciona com Option, numa função que devolve Option: None -> sai com None
fn inicial(nome: &str) -> Option<char> {
    let letra = nome.chars().next()?;
    Some(letra.to_ascii_uppercase())
}

fn propagar() {
    println!("{:?}", somar_textos("40", "2")); // -> Ok(42)
    println!("{:?}", somar_textos("40", "dois")); // -> Err(ParseIntError { kind: InvalidDigit })
    println!("{:?}", inicial("frieren")); // -> Some('F')
    println!("{:?}", inicial("")); // -> None
}

// --- 4. Erro próprio: o comando do jogador ---------------------------------

#[derive(Debug)]
enum Comando {
    Atacar(u32),
    Curar(u32),
    Fugir,
}

// Cada jeito de errar vira uma variante, com o texto que causou o erro
enum ErroComando {
    Vazio,
    Desconhecido(String),
    FaltaNumero(String),
    NumeroInvalido(String),
}

impl ErroComando {
    fn mensagem(&self) -> String {
        match self {
            ErroComando::Vazio => String::from("digite um comando"),
            ErroComando::Desconhecido(verbo) => format!("comando desconhecido: {verbo}"),
            ErroComando::FaltaNumero(verbo) => format!("`{verbo}` precisa de um número"),
            ErroComando::NumeroInvalido(texto) => format!("`{texto}` não é um número válido"),
        }
    }
}

fn ler_numero(verbo: &str, argumento: Option<&str>) -> Result<u32, ErroComando> {
    // `ok_or` transforma Option em Result: Some(x) -> Ok(x), None -> Err(o erro dado)
    let texto = argumento.ok_or(ErroComando::FaltaNumero(verbo.to_string()))?;
    let Ok(numero) = texto.parse::<u32>() else {
        return Err(ErroComando::NumeroInvalido(texto.to_string()));
    };
    Ok(numero)
}

// Lê "verbo número"; os `?` repassam para cima qualquer ErroComando
fn interpretar(linha: &str) -> Result<Comando, ErroComando> {
    let linha = linha.trim().to_lowercase();
    let mut partes = linha.split_whitespace(); // iterador de palavras
    let verbo = partes.next().ok_or(ErroComando::Vazio)?;
    match verbo {
        "atacar" => Ok(Comando::Atacar(ler_numero(verbo, partes.next())?)),
        "curar" => Ok(Comando::Curar(ler_numero(verbo, partes.next())?)),
        "fugir" => Ok(Comando::Fugir),
        outro => Err(ErroComando::Desconhecido(outro.to_string())),
    }
}

fn testar_comandos() {
    let linhas = [
        "atacar 30",
        "  CURAR 15 ",
        "fugir",
        "",
        "dançar",
        "atacar",
        "curar muito",
    ];
    for linha in linhas {
        match interpretar(linha) {
            Ok(comando) => println!("{linha:?} -> {comando:?}"),
            Err(erro) => println!("{linha:?} -> erro: {}", erro.mensagem()),
        }
    }
}

// --- 5. `?` no main: lendo o teclado ---------------------------------

// O main também pode devolver Result: um Err que chega aqui encerra o programa
fn main() -> Result<(), io::Error> {
    converter();
    atalhos();
    propagar();
    testar_comandos();

    println!("Um slime apareceu! Comandos: atacar N, curar N, fugir");
    let mut vida_slime: u32 = 50;
    let mut linha = String::new();
    loop {
        // `read_line` ACRESCENTA no fim da String: limpe antes de cada leitura
        linha.clear();
        // Devolve io::Result<usize> (bytes lidos); 0 = fim da entrada (Ctrl+D)
        if io::stdin().read_line(&mut linha)? == 0 {
            break;
        }

        // Padrões encaixados: Ok(...) por fora, a variante do Comando por dentro
        match interpretar(&linha) {
            Ok(Comando::Atacar(dano)) => {
                vida_slime = vida_slime.saturating_sub(dano);
                println!("Zoltraak! O slime ficou com {vida_slime} de vida");
                if vida_slime == 0 {
                    println!("Slime derrotado!");
                    break;
                }
            }
            Ok(Comando::Curar(cura)) => println!("Frieren recupera {cura} de vida"),
            Ok(Comando::Fugir) => {
                println!("Você fugiu.");
                break;
            }
            Err(erro) => println!("Erro: {}", erro.mensagem()),
        }
    }
    Ok(())
}
