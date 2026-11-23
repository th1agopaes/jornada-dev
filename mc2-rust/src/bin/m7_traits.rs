//! MC2 · Rust do zero — Módulo 7 · Aula 1: traits
//! Objetivo: definir um contrato de comportamento e cumpri-lo em tipos diferentes.
//! Rodar:    `cargo run --bin m7_traits`

// --- 1. `trait`: o contrato ---------------------------------

// Um trait é a lista de métodos que um tipo promete ter. Aqui, "tudo que luta":
// tem nome, tem vida e pode levar dano
trait Combatente {
    // Obrigatórios: só a assinatura, terminando em `;`. Cada tipo escreve o corpo
    fn nome(&self) -> &str;
    fn vida(&self) -> u32;
    fn vida_mut(&mut self) -> &mut u32; // empresta a vida para alterar (o `get_mut`)

    // Padrão (default method): o corpo vem pronto e vale para todos os tipos.
    // Ele não enxerga os campos, só os outros métodos do trait
    fn esta_vivo(&self) -> bool {
        self.vida() > 0
    }

    fn receber(&mut self, dano: u32) {
        let vida = self.vida_mut();
        *vida = vida.saturating_sub(dano);
    }

    fn grito(&self) -> String {
        format!("{} ataca!", self.nome())
    }
}

struct Heroi {
    nome: String,
    classe: String,
    vida: u32,
}

// Métodos próprios do tipo: o mesmo `impl` do Módulo 3
impl Heroi {
    fn new(nome: &str, classe: &str) -> Self {
        Self {
            nome: nome.to_string(),
            classe: classe.to_string(),
            vida: 100,
        }
    }
}

// Métodos do contrato: `impl Trait for Tipo`, só com os obrigatórios (os padrão vêm junto)
impl Combatente for Heroi {
    fn nome(&self) -> &str {
        &self.nome
    }

    fn vida(&self) -> u32 {
        self.vida
    }

    fn vida_mut(&mut self) -> &mut u32 {
        &mut self.vida
    }
}

fn contrato() {
    let mut frieren = Heroi::new("Frieren", "maga");
    frieren.receber(30); // método padrão, escrito uma vez só no trait
    let vida = frieren.vida();
    println!("{}, a {}: {vida} HP", frieren.nome(), frieren.classe);
    // -> Frieren, a maga: 70 HP
    println!("{}", frieren.grito()); // -> Frieren ataca!
}

// --- 2. O mesmo contrato em outro tipo ---------------------------------

// Campos diferentes dos do Heroi: o trait não olha o formato, só os métodos
struct Inimigo {
    nome: String,
    vida: u32,
    xp: u32,
}

impl Combatente for Inimigo {
    fn nome(&self) -> &str {
        &self.nome
    }

    fn vida(&self) -> u32 {
        self.vida
    }

    fn vida_mut(&mut self) -> &mut u32 {
        &mut self.vida
    }

    // Sobrescrever (override): o tipo troca o corpo de um método padrão pelo seu
    fn grito(&self) -> String {
        format!("{} ruge e avança!", self.nome)
    }
}

fn dois_tipos() {
    let mut mimico = Inimigo {
        nome: "Mímico".to_string(),
        vida: 40,
        xp: 25,
    };
    println!("{} ({} XP)", mimico.grito(), mimico.xp);
    // -> Mímico ruge e avança! (25 XP)
    mimico.receber(50); // o `saturating_sub` do método padrão para no 0
    println!("vivo? {}", mimico.esta_vivo()); // -> vivo? false
}

// --- 3. Trait como parâmetro: `impl Combatente` ---------------------------------

// `&impl Combatente` = "empresta qualquer tipo que cumpra o contrato".
// A mesma função serve para Heroi e Inimigo, nos dois papéis
fn golpear(atacante: &impl Combatente, alvo: &mut impl Combatente, dano: u32) {
    println!("{}", atacante.grito());
    alvo.receber(dano);
    if alvo.esta_vivo() {
        println!("  {} fica com {} HP", alvo.nome(), alvo.vida());
    } else {
        println!("  {} foi derrotado!", alvo.nome());
    }
}

fn arena() {
    let mut frieren = Heroi::new("Frieren", "maga");
    let mut mimico = Inimigo {
        nome: "Mímico".to_string(),
        vida: 60,
        xp: 25,
    };

    golpear(&mimico, &mut frieren, 15); // -> Mímico ruge e avança! / Frieren fica com 85 HP
    golpear(&frieren, &mut mimico, 35); // -> Frieren ataca! / Mímico fica com 25 HP
    golpear(&frieren, &mut mimico, 35); // -> Frieren ataca! / Mímico foi derrotado!
}

// --- 4. Trait no escopo: o E0599 do `RngExt` ---------------------------------

mod magia {
    pub trait Feitico {
        fn conjurar(&self) -> String;
    }

    // Trait deste módulo, tipo do módulo pai (`super::`). O campo `nome` é privado,
    // mas o filho enxerga o privado dos ancestrais (Módulo 6)
    impl Feitico for super::Heroi {
        fn conjurar(&self) -> String {
            format!("{} conjura Zoltraak!", self.nome)
        }
    }
}

fn escopo() {
    // O método de um trait só aparece com o trait no escopo, como o `random_range` do `RngExt`
    use crate::magia::Feitico;

    let frieren = Heroi::new("Frieren", "maga");
    println!("{}", frieren.conjurar()); // -> Frieren conjura Zoltraak!
}

fn main() {
    contrato();
    dois_tipos();
    arena();
    escopo();
}
