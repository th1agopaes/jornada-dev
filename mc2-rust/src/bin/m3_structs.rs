// Módulo 3 · Aula 1: structs (definir, criar, ler, alterar e copiar campos)

#[derive(Debug)]
struct Aluno {
    nome: String,
    curso: String,
    semestre: u8,
    ativo: bool,
}

// Tuple structs: os campos não têm nome, mas o TIPO tem
struct Celsius(f64);
struct Fahrenheit(f64);

// Recebe &str (só lê) e cria os String que a struct vai guardar
fn novo_aluno(nome: &str, curso: &str) -> Aluno {
    Aluno {
        nome: nome.to_string(),
        curso: String::from(curso),
        semestre: 1,
        ativo: true,
    }
}

// Recebe String (guarda): parâmetro com o mesmo nome do campo dispensa "nome: nome"
fn aluno_com(nome: String, curso: String, semestre: u8) -> Aluno {
    Aluno {
        nome,
        curso,
        semestre,
        ativo: true,
    }
}

fn para_fahrenheit(c: Celsius) -> Fahrenheit {
    Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
}

fn main() {
    // 1. Criar e ler campos
    let haru = Aluno {
        nome: String::from("Haru"),
        curso: String::from("Engenharia da Computação"),
        semestre: 5,
        ativo: true,
    };
    println!(
        "{} cursa {} ({}º semestre)",
        haru.nome, haru.curso, haru.semestre
    );
    println!("Matrícula ativa? {}", haru.ativo);

    // 2. mut vale para a struct inteira
    let mut bia = novo_aluno("Bia", "Física");
    bia.semestre += 1;
    bia.nome.push_str(" Souza");
    println!("{} agora está no {}º semestre", bia.nome, bia.semestre);

    // 3. Debug: {:?} em uma linha, {:#?} em várias
    let caio = aluno_com(String::from("Caio"), String::from("Matemática"), 3);
    println!("{caio:?}");
    println!("{caio:#?}");

    // 4. Struct update syntax: os campos que faltam vêm de caio
    let dani = Aluno {
        nome: String::from("Dani"),
        ..caio
    };
    println!("{} e {} cursam {}", caio.nome, dani.nome, dani.curso);
    println!(
        "{}: {}º · {}: {}º",
        caio.nome, caio.semestre, dani.nome, dani.semestre
    );

    // 5. Tuple structs
    let agora = Celsius(31.5);
    println!("{:.1} °C", agora.0);
    let f = para_fahrenheit(agora);
    println!("{:.1} °F", f.0);
}
