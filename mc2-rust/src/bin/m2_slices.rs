// Módulo 2 · Parte C: slices (&str e &[T])

fn main() {
    // 1. Fatias de uma String com ranges
    let frase = String::from("ownership em rust");
    let comeco = &frase[0..9];
    let fim = &frase[13..];
    let tudo = &frase[..];
    println!("comeco = '{comeco}', fim = '{fim}', tudo = '{tudo}'");

    // 2. Função que devolve uma fatia de quem chamou
    let palavra = primeira_palavra(&frase);
    println!("primeira palavra: {palavra}");
    // frase.clear(); // E0502: `palavra` ainda empresta `frase`

    // 3. Literal também é &str: a mesma função serve
    println!("primeira palavra: {}", primeira_palavra("borrow checker"));

    // 4. Fatias de array: &[T] de qualquer tamanho
    let notas = [7.0, 8.0, 5.0, 9.0];
    println!("média de todas: {:.2}", media(&notas));
    println!("média das 2 últimas: {:.2}", media(&notas[2..]));

    // 5. len() conta bytes; char() conta caracteres
    let acao = "ação";
    println!(
        "'{acao}': {} bytes, {} chars",
        acao.len(),
        acao.chars().count()
    );
}

fn primeira_palavra(texto: &str) -> &str {
    for (i, letra) in texto.char_indices() {
        if letra == ' ' {
            return &texto[..i];
        }
    }
    texto
}

fn media(valores: &[f64]) -> f64 {
    let mut soma = 0.0;
    for v in valores {
        soma += v; // v é &f64: percorrer uma fatia entrega referências
    }
    soma / valores.len() as f64
}
