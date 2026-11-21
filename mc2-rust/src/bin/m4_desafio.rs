//! MC2 · Rust do zero - Módulo 4 · Desafio: a masmorra da Frieren
//! Objetivo: praticar enum com dados, `match` (padrões, `|`, guardas, tuplas), `Option`,
//! `if let`, `let ... else` e `take()`.
//!
//! Rodar: cargo run --bin m4_desafio

// --- 1. Itens e elementos ------------------------------------------------------

#[derive(Debug)]
enum Elemento {
    Fogo,
    Gelo,
    Raio,
}

#[derive(Debug)]
enum Item {
    Pocao(u32), // quanto cura
    // o pergaminho guarda um elemento e o poder
    Pergaminho { elemento: Elemento, poder: u32 },
    Chave(String), // nome da porta que abre
}

impl Item {
    // (`elemento`, `poder`, `porta`) mostram quais bindings o padrão precisa criar
    // `Pergaminho` tem campos com nome (padrão com `{ }`); `Chave` é como tupla.
    fn descrever(&self) -> String {
        match self {
            Item::Pocao(cura) => format!("Poção (+{cura})"),
            Item::Pergaminho { elemento, poder } => format!("Pergaminho de {elemento:?} ({poder})"),
            Item::Chave(porta) => format!("Chave da porta {porta}"),
        }
    }
}

// --- 2. Fraquezas: `match` em dupla ------------------------------------------------------

// ataque forte -> 2; mesmo elemento -> 0 (imune); o resto -> 1.
// Fogo vence Gelo, Gelo vence Raio, Raio vence Fogo.
// `match (ataque, defesa)` com três braços: as 3 duplas fortes juntas com `|`, as 3
// iguais juntas com `|` e o `_` para o resto.
fn multiplicador(ataque: &Elemento, defesa: &Elemento) -> u32 {
    match (ataque, defesa) {
        (Elemento::Fogo, Elemento::Gelo)
        | (Elemento::Gelo, Elemento::Raio)
        | (Elemento::Raio, Elemento::Fogo) => 2,
        (Elemento::Fogo, Elemento::Fogo)
        | (Elemento::Gelo, Elemento::Gelo)
        | (Elemento::Raio, Elemento::Raio) => 0,
        _ => 1,
    }
}

// --- 3. Estados do herói -------------------------------------------------------

#[derive(Debug)]
enum Estado {
    Normal,
    Envenenado(u32), // dano por turno
    Congelado,
}

impl Estado {
    // `Envenenado(dano)` devolve o dano; os outros estados, 0.
    // o match é em `&self`, então o binding é `&u32`: lembrar do `*`.
    fn dano_por_turno(&self) -> u32 {
        match self {
            Estado::Envenenado(dano) => *dano,
            _ => 0,
        }
    }

    // só o `Congelado` não age. Uma linha com `matches`.
    fn pode_agir(&self) -> bool {
        !matches!(self, Estado::Congelado)
    }
}

// --- 4. Procurando na mochila: `Option` como retorno -------------------------------------------------------

// devolve a cura da PRIMEIRA poção da mochila, ou `None` se não houver nenhuma.
// `for` + `if let Item::Pocao(cura) = item { return ... }`. Se o laço terminar, é porque
// nenhuma poção apareceu.
fn primeira_pocao(mochila: &[Item]) -> Option<u32> {
    for item in mochila {
        if let Item::Pocao(cura) = item {
            return Some(*cura);
        }
    }
    None
}

// --- 5. O herói: `Option` em campo, `take()` e `let ... else` -------------------------------------------------------

const VIDA_MAX: u32 = 100;

struct Heroi {
    nome: String,
    vida: u32,
    estado: Estado,
    // o item na mão pode existir ou não
    mao: Option<Item>,
}

impl Heroi {
    fn new(nome: &str, vida: u32) -> Self {
        Self {
            nome: nome.to_string(),
            vida,
            estado: Estado::Normal,
            mao: None,
        }
    }

    // usa (e gasta) o item da mão. Textos exatos (`{nome}` = `self.nome`):
    // mão vazia -> "{nome} está de mãos vazias"
    // Pocao(cura) -> soma na vida sem passar de VIDA_MAX (`.min`)
    // -> "{nome} bebe a poção: vida {vida}"
    // Pergaminho poder >=50 -> "{nome} lança {elemento:?} CRÍTICO ({poder})"
    // outro Pergaminho -> "{nome} lança {elemento:?} ({poder})"
    // Chave(porta) -> "{nome} abre a porta {porta}"
    // `let Some(item) = self.mao.take() else { return ... };` resolve a mão vazia.
    // depois do `take()`, `item` é um `Item` dono (sem `&`): o `match item` entrega os
    // valores direto, sem `*`. O braço com guarda vem antes do Pergaminho genérico.
    fn usar_mao(&mut self) -> String {
        let Some(item) = self.mao.take() else {
            return format!("{} está de mãos vazias", self.nome);
        };

        match item {
            Item::Pocao(cura) => {
                self.vida = self.vida.saturating_add(cura).min(VIDA_MAX);
                format!("{} bebe a poção: vida {}", self.nome, self.vida)
            }
            Item::Pergaminho { elemento, poder } if poder >= 50 => {
                format!("{} lança {elemento:?} CRÍTICO ({poder})", self.nome)
            }
            Item::Pergaminho { elemento, poder } => {
                format!("{} lança {elemento:?} ({poder})", self.nome)
            }
            Item::Chave(porta) => {
                format!("{} abre a porta {porta}", self.nome)
            }
        }
    }

    // aplica o dano do estado no fim do turno.
    // sem dano (0) -> None
    // com dano -> tira a vida sem estourar (`saturating_sub`) e devolve
    // Some("{nome} sofre {dano} de veneno: vida {vida}")
    // reaproveitar o `dano_por_turno`
    fn fim_de_turno(&mut self) -> Option<String> {
        let dano = self.estado.dano_por_turno();
        if dano == 0 {
            return None;
        }
        self.vida = self.vida.saturating_sub(dano);
        Some(format!(
            "{} sofre {dano} de veneno: vida {}",
            self.nome, self.vida
        ))
    }
}

// --- 6. Testando tudo -------------------------------------------------------

fn main() {
    // 1. Itens
    let mochila = [
        Item::Chave(String::from("Cripta")),
        Item::Pergaminho {
            elemento: Elemento::Fogo,
            poder: 40,
        },
        Item::Pocao(25),
    ];
    for item in &mochila {
        println!("{}", item.descrever());
    }
    // -> Chave da porta Cripta
    // -> Pergaminho de Fogo (40)
    // -> Poção (+25)

    // 2. Fraquezas
    let fogo_gelo = multiplicador(&Elemento::Fogo, &Elemento::Gelo);
    let raio_raio = multiplicador(&Elemento::Raio, &Elemento::Raio);
    let gelo_fogo = multiplicador(&Elemento::Gelo, &Elemento::Fogo);
    println!("Fogo x Gelo: {fogo_gelo} | Raio x Raio: {raio_raio} | Gelo x Fogo: {gelo_fogo}");
    // -> Fogo x gelo: 2 | Raio x Raio: 0 | Gelo x Fogo: 1

    // 3. Estados
    for estado in [Estado::Normal, Estado::Envenenado(7), Estado::Congelado] {
        println!(
            "{estado:?}: dano/turno {}, age? {}",
            estado.dano_por_turno(),
            estado.pode_agir()
        );
    }
    // -> Normal: dano/turno 0, age? true
    // -> Envenenado(7): dano/turno 7, age? true
    // -> Congelado: dano/turno 0, age? false

    // 4. Procurando na mochila
    println!("{:?}", primeira_pocao(&mochila)); // -> Some(25)
    println!("{}", primeira_pocao(&mochila[..2]).unwrap_or(0)); // -> 0 (só chave e pergaminho)

    // 5. Usando a mão
    let mut frieren = Heroi::new("Frieren", 80);
    println!("{}", frieren.usar_mao()); // -> Frieren está de mãos vazias
    frieren.mao = Some(Item::Pocao(30));
    println!("{}", frieren.usar_mao()); // -> Frieren bebe a poção: vida 100
    println!("{}", frieren.usar_mao()); // -> Frieren está de mãos vazias (o `take` gastou)
    frieren.mao = Some(Item::Pergaminho {
        elemento: Elemento::Gelo,
        poder: 30,
    });
    println!("{}", frieren.usar_mao()); // -> Frieren lança Gelo (30)
    frieren.mao = Some(Item::Pergaminho {
        elemento: Elemento::Raio,
        poder: 55,
    });
    println!("{}", frieren.usar_mao()); // -> Frieren lança Raio CRÍTICO (55)
    frieren.mao = Some(Item::Chave(String::from("Cripta")));
    println!("{}", frieren.usar_mao()); // -> Frieren abre a porta Cripta

    // 6. Fim de turno
    frieren.estado = Estado::Envenenado(7);
    if let Some(aviso) = frieren.fim_de_turno() {
        println!("{aviso}"); // -> Frieren sofre 7 de veneno: vida 93
    }
    frieren.estado = Estado::Normal;
    println!("{:?}", frieren.fim_de_turno()); // -> None
}
