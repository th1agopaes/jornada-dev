fn main() {
    // 1. inteiros: sinal (i/u) + tamanho em bits
    let idade: u8 = 21;
    let saldo: i32 = -150;
    let populacao: u64 = 8_200_000_000;
    let inferido = 42;
    let decimal = 2.5;
    println!("{idade} {saldo} {populacao} {inferido} {decimal}");

    // 2. bool e char
    let aprovado = true;
    let letra = 'A';
    let caranguejo = '🦀';
    println!("{aprovado} {letra} {caranguejo}");

    // 3. contas: o tipo decide a divisão
    println!("7 / 2 = {}", 7 / 2);
    println!("7.0 / 2.0 = {}", 7.0 / 2.0);
    println!("7 % 2 = {}", 7 % 2);

    // 4. conversão sempre explícita
    let total: i32 = 17;
    let quantidade: i32 = 4;
    let media = total as f64 / quantidade as f64;
    println!("média = {media} | com 3 casas: {media:.3}");

    // 5. tupla: tamanho fixo, tipos misturados
    let aluno = ("Haru", 5, 8.5);
    let (nome, semestre, nota) = aluno;
    println!("{nome} está no {semestre}º semestre e tem nota {nota}");
    println!("pela posição: {} e {}", aluno.0, aluno.2);

    // 6. array: tamanho fixo, um tipo só
    let notas: [f64; 4] = [8.5, 7.0, 9.25, 6.0];
    let zeros = [0; 3];
    println!("{:?} tem {} notas; a primeira é {}", notas, notas.len(), notas[0]);
    println!("{zeros:?}");
}
