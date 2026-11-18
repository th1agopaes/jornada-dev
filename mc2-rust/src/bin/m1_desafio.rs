const MEDIA_APROVACAO: f64 = 7.0;

fn main() {
    let boletim = [
        ("Compiladores", [8.5, 7.0, 9.5, 6.0]),
        ("Engenharia de Software", [5.0, 6.5, 4.0, 6.5]),
        ("Programação Web", [3.0, 4.5, 2.0, 4.5]),
        ("Extensão I", [10.0, 9.0, 9.5, 9.5]),
    ];

    println!("== Boletim ==");
    let mut aprovadas = 0;
    for (disciplina, notas) in boletim {
        let m = media(notas);
        let situacao = if aprovado(m) { "aprovado" } else { "em risco" };
        println!(
            "{disciplina}: média {m:.2}, conceito {}, {situacao}",
            conceito(m)
        );
        if aprovado(m) {
            aprovadas += 1;
        }
    }
    println!("aprovado em {aprovadas} de {}", boletim.len());

    println!("== Fibonacci ==");
    for n in [0, 1, 2, 10, 50] {
        println!("fib({n}) = {}", fibonacci(n));
    }
}

fn media(notas: [f64; 4]) -> f64 {
    let mut soma = 0.0;
    for nota in notas {
        soma += nota;
    }
    soma / notas.len() as f64
}

fn aprovado(media: f64) -> bool {
    media >= MEDIA_APROVACAO
}

fn conceito(media: f64) -> char {
    if media >= 9.0 {
        'A'
    } else if media >= MEDIA_APROVACAO {
        'B'
    } else if media >= 5.0 {
        'C'
    } else {
        'D'
    }
}

fn fibonacci(n: u32) -> u64 {
    let mut atual = 0;
    let mut proximo = 1;
    for _ in 0..n {
        let temp = atual;
        atual = proximo;
        proximo += temp;
    }
    atual
}
