//! MC2 · Rust do zero — Módulo 7 · Aula 4: lifetimes
//! Objetivo: dizer ao compilador por quanto tempo uma referência devolvida continua valendo.
//! Rodar:    `cargo run --bin m7_lifetimes`

// --- 1. Duas entradas, uma saída: `'a` ---------------------------------

struct Inimigo {
    nome: String,
    ataque: u32,
}

impl Inimigo {
    fn new(nome: &str, ataque: u32) -> Self {
        Self {
            nome: nome.to_string(),
            ataque,
        }
    }
}

// O retorno empresta de `a` OU de `b`, e qual dos dois só se sabe ao rodar. `'a` é um
// parâmetro de lifetime: "o retorno vale enquanto `a` e `b` valerem"
fn mais_forte<'a>(a: &'a Inimigo, b: &'a Inimigo) -> &'a Inimigo {
    if a.ataque >= b.ataque { a } else { b }
}

fn duas_entradas() {
    let dragao = Inimigo::new("Dragão", 80);
    let slime = Inimigo::new("Slime", 5);
    let vencedor = mais_forte(&dragao, &slime);
    println!("Mais forte: {}", vencedor.nome); // -> Mais forte: Dragão
}

// --- 2. O `'a` é o menor dos dois prazos ---------------------------------

fn prazo() {
    let dragao = Inimigo::new("Dragão", 80);
    {
        let slime = Inimigo::new("Slime", 5);
        let vencedor = mais_forte(&dragao, &slime);
        println!("No bloco: {}", vencedor.nome); // -> No bloco: Dragão
    } // drop do `slime`: daqui em diante, nenhum retorno do `mais_forte` com ele vale

    // Se o `vencedor` fosse declarado fora do bloco e usado aqui: E0597 (`slime` does not
    // live long enough), mesmo sendo o Dragão. O compilador confere a assinatura, não o
    // valor que veio ao rodar
    println!("{} continua de pé", dragao.nome); // -> Dragão continua de pé
}

// --- 3. Elisão: quando o `'a` não aparece ---------------------------------

// Uma referência de entrada só: o compilador deduz que o retorno vem dela (elisão).
// É o `primeira_palavra` do Módulo 2, sem `'a` escrito
fn primeira_palavra(frase: &str) -> &str {
    frase.split_whitespace().next().unwrap_or("")
}

// Duas entradas, mas o retorno só pode vir do `texto`: só ele ganha o `'a`.
// O `marcador` pode sumir logo depois da chamada
fn depois_de<'a>(texto: &'a str, marcador: &str) -> &'a str {
    match texto.split_once(marcador) {
        Some((_, resto)) => resto,
        None => texto,
    }
}

fn elisao() {
    println!("{}", primeira_palavra("Zoltraak, o feitiço que mata")); // -> Zoltraak,

    let fala = String::from("Frieren: Zoltraak!");
    let resto;
    {
        let marcador = String::from(": ");
        resto = depois_de(&fala, &marcador);
    } // drop do `marcador`; o `resto` continua valendo porque só depende da `fala`
    println!("{resto}"); // -> Zoltraak!
}

// --- 4. Struct que empresta: `Fala<'a>` ---------------------------------

// A fala não copia o texto: aponta para dentro do roteiro. `<'a>` na struct =
// "uma Fala não pode viver mais que o texto de onde ela empresta"
struct Fala<'a> {
    personagem: &'a str,
    texto: &'a str,
}

// `'_` = "um lifetime que o compilador deduz" (o `Formatter<'_>` da Aula 3). Aqui ele
// vem da `linha`: a Fala devolvida empresta dela
fn ler_fala(linha: &str) -> Option<Fala<'_>> {
    let (personagem, texto) = linha.split_once(": ")?;
    Some(Fala { personagem, texto })
}

impl Fala<'_> {
    fn mostrar(&self) -> String {
        format!("{} diz \"{}\"", self.personagem, self.texto)
    }
}

fn roteiro() {
    let roteiro = String::from("Frieren: Vamos.\nFern: Sim, senhora Frieren.\n(silêncio)");
    for linha in roteiro.lines() {
        match ler_fala(linha) {
            Some(fala) => println!("{}", fala.mostrar()), // -> Frieren diz "Vamos." ...
            None => println!("[cena] {linha}"),           // -> [cena] (silêncio)
        }
    }
}

fn main() {
    duas_entradas();
    prazo();
    elisao();
    roteiro();
}
