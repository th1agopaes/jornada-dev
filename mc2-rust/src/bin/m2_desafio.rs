// Módulo 2 · Exercício: analisador de anotações (ownership, &, &mut e slices)

fn main() {
    let anotacao = String::from("ownership evita vazamento e double free");

    // Etapa 1
    println!("vogais: {}", contar_vogais(&anotacao));

    // Etapa 2
    println!("maior palavra: {}", maior_palavra(&anotacao));

    // Etapa 3
    let notas = [6.5, 9.0, 7.5, 8.0];
    println!("maior nota: {:.1}", maior_nota(&notas));
    println!("maior nota das 2 últimas: {:.1}", maior_nota(&notas[2..]));

    // Etapa 4
    let titulo = juntar("MC2", "Rust");
    println!("título: {titulo}");

    // Etapa 5
    let mut aviso = String::from("compilou");
    gritar(&mut aviso);
    println!("aviso: {aviso}");

    // Etapa 6
    let mut resumo = juntar(&titulo, "· ownership");
    gritar(&mut resumo);
    let copia = &resumo;
    println!(
        "resumo: {resumo} | cópia: {copia} | vogais: {}",
        contar_vogais(&resumo)
    );
}

// Etapa 1: conta a, e, i, o, u (maiúsculas ou minúsculas, sem acento).
// `texto.chars()` percorre os caracteres; `"aeiouAEIOU".contains(letra)` diz se é vogal.
fn contar_vogais(texto: &str) -> usize {
    let mut total = 0;
    for letra in texto.chars() {
        if "aeiouAEIOU".contains(letra) {
            total += 1;
        }
    }
    total
}

// Etapa 2: devolve a palavra mais longa como fatia do próprio `texto`. Em empate, fica a primeira.
// `texto.split(' ')` entrega cada palavra como `&str`. A comparação é `>` ou `>=`.
fn maior_palavra(texto: &str) -> &str {
    let mut maior = "";
    for palavra in texto.split_whitespace() {
        if palavra.chars().count() > maior.len() {
            maior = palavra;
        }
    }
    maior
}

// Etapa 3: maior valor de uma fatia não vazia.
// percorrer `&[f64]` entrega `&f64`; usar `*` para chegar ao número.
fn maior_nota(notas: &[f64]) -> f64 {
    let mut maior = notas[0];
    for n in notas {
        if *n > maior {
            maior = *n;
        }
    }
    maior
}

// Etapa 4: cria uma String NOVA com `a`, um espaço e `b`
// `String::from(a)` + `push(' ')` + `push_str(b)`, ou `format!`, que é o `println!` que devolve String.
fn juntar(a: &str, b: &str) -> String {
    let mut s = String::from(a);
    s.push(' ');
    s.push_str(b);
    s
}

// Etapa 5: deixa o texto em maiúsculas e põe '!' no fim, alterando o original.
// `texto.to_uppercase()` devolve uma String nova; para trocar o valor emprestado, usa `*texto = ...`.
fn gritar(texto: &mut String) {
    *texto = texto.to_uppercase();
    texto.push('!');
}
