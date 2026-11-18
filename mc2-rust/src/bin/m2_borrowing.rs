// Módulo 2 · Parte B: funções e ownership, referências & e &mut

fn main() {
    // 1. Passar String por valor = move para dentro da função
    let s = String::from("rust");
    let tamanho = consumir(s);
    println!("consumir devolveu {tamanho}");
    // println!("{s}"); // E0382 de novo: `s` foi movida para `consumir`

    // 2. Devolver a posse no retorno (funciona, mas é trabalhoso)
    let s1 = String::from("ida");
    let s2 = ida_e_volta(s1);
    println!("voltou: {s2}");

    // 3. Referência &: empresta só para ler
    let curso = String::from("MC2 Rust");
    let n = contar_bytes(&curso);
    println!("'{curso}' tem {n} bytes"); // `curso` continua dona

    // 4. Referência &mut: empresta para alterar
    let mut lista = String::from("ownership");
    adicionar(&mut lista, "borrowing");
    adicionar(&mut lista, "slices");
    println!("lista: {lista}");

    // 5. Várias & ao mesmo tempo: pode
    let r1 = &lista;
    let r2 = &lista;
    println!("r1 = {r1}, r2 = {r2}");
    // r1 e r2 não são usadas daqui para baixo: o empréstimo acabou

    // 6. Agora cabe uma &mut
    let r3 = &mut lista;
    r3.push('!');
    println!("r3 = {r3}");

    // 7. & e * com inteiros, como em C
    let mut contador = 0;
    incrementar(&mut contador);
    incrementar(&mut contador);
    println!("contador = {contador}");
}

fn consumir(texto: String) -> usize {
    texto.len()
} // `texto` sai do escopo aqui: o drop libera o heap

fn ida_e_volta(texto: String) -> String {
    texto // devolve a posse para quem chamou
}

fn contar_bytes(texto: &str) -> usize {
    texto.len()
}

fn adicionar(lista: &mut String, item: &str) {
    lista.push_str(", ");
    lista.push_str(item);
}

fn incrementar(n: &mut i32) {
    *n += 1; // * = "o valor para onde n aponta"
}
