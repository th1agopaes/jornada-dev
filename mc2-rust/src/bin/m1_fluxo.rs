fn main() {
    // 1. if / else if / else: a condição tem que ser bool
    let nota = 6.5;
    if nota >= 7.0 {
        println!("aprovado");
    } else if nota >= 5.0 {
        println!("recuperação");
    } else {
        println!("reprovado");
    }

    // 2. if é expressão (no lugar do ternário `? : `)
    let situacao = if nota >= 7.0 { "aprovado" } else { "em risco" };
    println!("situação: {situacao}");

    // 3. loop: infinito até o break, que pode devolver um valor
    let mut contador = 0;
    let resultado = loop {
        contador += 1;
        if contador == 10 {
            break contador * 2;
        }
    };
    println!("resultado do loop: {resultado}");

    // 4. while:
    let mut n = 3;
    while n > 0 {
        println!("{n}...");
        n -= 1;
    }
    println!("decolar!");

    // 5. for percorre uma coleção (não existe o for de 3 partes do C)
    let notas = [8.5, 7.0, 9.25, 6.0];
    let mut soma = 0.0;
    for nota in notas {
        soma += nota;
    }
    println!("soma = {soma}");

    // 6. ranges (intervalos)
    for i in 1..4 {
        print!("{i} ");
    }
    println!("<- 1..4 (o 4 fica de fora)");
    for i in 1..=4 {
        print!("{i} ");
    }
    println!("<- 1..=4 (o 4 entra)");
    for i in (1..=3).rev() {
        print!("{i} ");
    }
    println!("<- (1..=3).rev()");
}
