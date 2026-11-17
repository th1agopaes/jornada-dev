const NOTA_MINIMA: f64 = 7.0;

fn main() {
    // 1. imutável por padrão; `mut` libera a troca
    let nome = "Haru";
    let mut tentativas = 0;
    println!("{nome} começou com {tentativas} tentativas");

    tentativas = tentativas + 1;
    tentativas += 1;
    println!("agora são {tentativas}");

    // 2. shadowing: um `let` novo com o mesmo nome
    let espacos = "    ";
    let mut espacos = espacos.len();
    println!("espaços: {espacos}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("dentro do bloco: {x}");
    }
    println!("fora do bloco: {x}");

    // 3. constantes
    println!("nota mínima: {NOTA_MINIMA:.1}");
}
