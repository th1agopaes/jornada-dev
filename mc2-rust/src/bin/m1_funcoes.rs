fn main() {
    saudar("Haru");

    let soma = somar(2, 3);
    println!("2 + 3 = {soma}");
    println!("dobro de 21 = {}", dobro(21));
    println!("|-5| = {} e |8| = {}", absoluto(-5), absoluto(8));

    // um bloco { } também é expressão: vale a última linha sem `;`
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {y}");
}

fn saudar(nome: &str) {
    println!("Olá, {nome}!");
}

fn somar(a: i32, b: i32) -> i32 {
    a + b
}

fn dobro(x: i32) -> i32 {
    x * 2
}

fn absoluto(n: i32) -> i32 {
    if n < 0 {
        return -n;
    }
    n
}
