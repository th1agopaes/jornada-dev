//! MC2 · Rust do zero - Módulo 5 · Aula 1: `Vec<T>`, a mochila que cresce
//! Objetivo: guardar uma lista de tamanho variável, ler sem panic e percorrer sem mover.
//! Rodar: `cargon run --bin m5_vec`

// --- 1. Criando um Vec ------------------------------------------------

fn criar() -> Vec<String> {
    // Vazio, o compilador não tem como deduzir o T: a anotação é obrigatória
    let vazia: Vec<String> = Vec::new();
    println!("{vazia:?} len={}", vazia.len()); // -> [] len=0

    // `vec![...]` cria já preenchido e deduz o tipo pelos itens: Vec<i32>
    let danos = vec![12, 30, 7];
    println!("{danos:?} len={}", danos.len()); // -> [12, 30, 7] len=3

    // Devolver o Vec passa a posse para quem chamou (Módulo 2): os itens não são copiados
    vec![String::from("Grimório"), String::from("Cajado")]
}

// --- 2. Crescendo e encolhendo ------------------------------------------------

fn crescer(mochila: &mut Vec<String>) {
    // `push` põe no fim; se a capacidade acabar, o Vec pede um espaço maior e muda de lugar
    mochila.push(String::from("Poção"));
    mochila.push(String::from("Éter"));
    println!("{mochila:?}"); // -> ["Grimório", "Cajado", "Poção", "Éter"]

    // `insert(i, x)` encaixa na posição i e empurra o resto para a direita
    mochila.insert(0, String::from("Mapa"));

    // `remove(i)` tira da posição i e devolve o item (com a posse); i inválido da panic
    let cajado = mochila.remove(2);
    println!("Saiu: {cajado}"); // -> Saiu: Cajado

    // `pop` tira o último e devolve Option<String>: None se estiver vazia (Módulo 4)
    let ultimo = mochila.pop();
    println!("{ultimo:?}"); // -> Some("Éter")
    println!("{} itens: {mochila:?}", mochila.len()); // -> 3 itens: ["Mapa", "Grimório", "Poção"]

    // ❌ E0502: `primeiro` empresta a mochila e o `push` pode mudar tudo de lugar no heap
    // let primeiro = &mochila[0];
    // mochila.push(String::from("Elixir"));
    // println!("{primeiro}");
}

// --- 3. Lendo: [i] ou get(i) ------------------------------------------------

fn ler(mochila: &[String]) {
    // `[i]` entrega o item direto, mas índice fora dá panic
    println!("{}", mochila[0]); // -> Mapa
    // mochila[10] -> panic: index out of bounds: the len is 3 but the index is 10

    // `get(i)` devolve Option<&String>: o jeito sem panic
    println!("{:?}", mochila.get(1)); // -> Some("Grimório")
    println!("{:?}", mochila.get(10)); // -> None

    if let Some(item) = mochila.get(2) {
        println!("Na posição 2: {item}"); // -> Na posição 2: Poção
    }

    // `first` e `last` também devolvem Option (vazia -> None)
    let Some(ultimo) = mochila.last() else {
        println!("Mochila vazia");
        return;
    };
    println!("Último: {ultimo}"); // -> Último: Poção
}

// ---4. Percorrendo: emprestar, alterar ou consumir ------------------------------------------------

fn percorrer(mochila: Vec<String>) {
    // `for x in &v` empresta: cada x é &String e a mochila continua valendo depois
    for item in &mochila {
        print!("[{item}] ");
    }
    println!(); // -> [Mapa] [Grimório] [Poção]

    // `.iter()` = o mesmo empréstimo; `.enumerate()` junta a posição: (0, &item), (1, &item)...
    for (i, item) in mochila.iter().enumerate() {
        println!("{}. {item}", i + 1); // -> 1. Mapa 2. Grimório 3. Poção (uma por linha)
    }

    // `for x in &mut v` altera no lugar: cada x é &mut i32 e o `*` chega no valor
    let mut durabilidade = vec![100, 80, 40];
    for d in &mut durabilidade {
        *d -= 10;
    }
    println!("{durabilidade:?}"); // -> [90, 70, 30]

    // `for x in v` consome: cada x é String (com a posse) e a mochila  deixa de existir
    let mut bau = Vec::new(); // sem anotação: o `push` abaixo mostra que é Vec<String>
    for item in mochila {
        bau.push(item); // a String muda de dono sem .clone()
    }
    println!("Baú: {bau:?}"); // -> Baú: ["Mapa", "Grimório", "Poção"]
    // println!("{mochila:?}"); // E0382: a mochila foi movida para dentro do `for`
}

// --- 5. Vec de enum: itens diferentes na mesma mochila -----------------------------------------------

// Todas as variantes são do tipo `Item`, então cabem no mesmo Vec (Módulo 4)
enum Item {
    Pocao(u32),
    Arma { nome: String, dano: u32 },
    Chave,
}

// Só lê -> `&[Item]`: aceita &Vec<Item>, array e pedaço de Vec
fn listar(mochila: &[Item]) {
    for item in mochila {
        let texto = match item {
            Item::Pocao(cura) => format!("Poção (+{cura} HP)"),
            Item::Arma { nome, dano } => format!("{nome} ({dano} de dano)"),
            Item::Chave => String::from("Chave"),
        };
        println!("- {texto}");
    }
}

fn cura_total(mochila: &[Item]) -> u32 {
    let mut total = 0;
    for item in mochila {
        if let Item::Pocao(cura) = item {
            total += *cura; // `cura` é &u32 (match ergonomics, Módulo 4)
        }
    }
    total
}

// --- 6. Juntando tudo ------------------------------------------------

fn main() {
    let mut mochila = criar();
    crescer(&mut mochila);
    ler(&mochila); // &Vec<String> vira &[String] sozinho
    percorrer(mochila); // a posse vai junto: daqui para baixo, `mochila` não existe

    let bolsa = vec![
        Item::Pocao(30),
        Item::Arma {
            nome: String::from("Cajado"),
            dano: 25,
        },
        Item::Chave,
        Item::Pocao(50),
    ];
    listar(&bolsa);
    println!("Cura total: {}", cura_total(&bolsa)); // -> Cura total: 80
    println!("Só as 2 primeiras: {}", cura_total(&bolsa[..2])); // -> Só as 2 primeiras: 30
}
