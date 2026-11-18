// Módulo 2 · Parte A: memória, String x &str e ownership (move, Copy, clone)

fn main() {
    // 1. &str: texto fixo, gravado dentro do executável
    let fixo: &str = "Haru";
    saudar(fixo);

    // 2. String: dona de um texto no heap, que pode crescer
    let mut nome = String::from("Ha");
    nome.push_str("ru");
    println!("nome = {nome}, len = {} bytes", nome.len());
    saudar(&nome); // empresta a String; o Rust converte &String em &str

    // 3. Move: a posse do texto passa de `a` para `b`
    let a = String::from("texto");
    let b = a;
    println!("b = {b}");
    // println!("a = {a}"); // E0382: `a` não é mais dona de nada

    // 4. Copy: tipos simples, só na stack, são copiados
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");

    // 5. clone: cópia explícita (e cara) do texto no heap
    let c = b.clone();
    println!("b = {b}, c = {c}");

    // 6. Escopo: ao fechar a chave, o dono sai de cena e o texto é liberado
    {
        let temporario = String::from("só existo aqui dentro");
        println!("{temporario}");
    } // aqui o Rust chama o drop de `temporario` (o "free" automático)
}

fn saudar(nome: &str) {
    println!("Olá, {nome}!");
}
