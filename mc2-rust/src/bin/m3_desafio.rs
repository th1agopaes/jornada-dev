// Módulo 3 · Exercício: arena de RPG com structs e métodos

// tuple structs
struct Dano(u32);
struct Cura(u32);

// structs
struct Heroi {
    nome: String,
    classe: String,
    vida: u32,
    vida_max: u32,
    ataque: u32,
    nivel: u8,
    xp: u32,
}

struct Inimigo {
    nome: String,
    vida: u32,
    ataque: u32,
    xp: u32,
}

struct Resumo {
    nome: String,
    nivel: u8,
    xp: u32,
}

const XP_POR_NIVEL: u32 = 100;

impl Heroi {
    // Começa com 100/100 de vida, ataque 12, nível 1 e 0 de XP.
    fn new(nome: &str, classe: &str) -> Self {
        Self {
            nome: nome.to_string(),
            classe: classe.to_string(),
            vida: 100,
            vida_max: 100,
            ataque: 12,
            nivel: 1,
            xp: 0,
        }
    }

    // Empresta o nome sem copiar.
    fn nome(&self) -> &str {
        &self.nome
    }

    // Formato: "Lyra, a Maga (nv. 1) · 100/100 HP · 0 XP"
    fn status(&self) -> String {
        format!(
            "{}, a {} (nv. {}) · {}/{} HP · {} XP",
            self.nome, self.classe, self.nivel, self.vida, self.vida_max, self.xp
        )
    }

    fn esta_vivo(&self) -> bool {
        self.vida > 0
    }

    // O herói não muda ao atacar; quem muda é o alvo.
    // chamar o recebedor do alvo com um Dano criado a partir do ataque
    fn atacar(&self, alvo: &mut Inimigo) {
        let dano = Dano(self.ataque);
        alvo.receber(dano);
    }

    // o número fica em dano.0; a vida não pode estourar abaixo de 0.
    fn receber(&mut self, dano: Dano) {
        self.vida = self.vida.saturating_sub(dano.0);
    }

    // A vida não passa de vida_max.
    fn curar(&mut self, cura: Cura) {
        self.vida = self.vida.saturating_add(cura.0).min(self.vida_max);
    }

    // Soma o XP. Enquanto xp >= XP_POR_NIVEL: tira 100 do xp, nível +1,
    // vida_max +10, vida + 10, ataque +3 e imprime "Lyra subiu para o nível 2".
    fn ganhar_xp(&mut self, xp: u32) {
        self.xp += xp;
        while self.xp >= XP_POR_NIVEL {
            self.xp -= XP_POR_NIVEL;
            self.nivel += 1;
            self.vida_max += 10;
            self.vida = (self.vida + 10).min(self.vida_max);
            self.ataque += 3;
            println!("{} subiu para o nível {}!", self.nome, self.nivel);
        }
    }

    // Fim da jornada: o Heroi deixa de existir e vira um Resumo.
    fn encerrar(self) -> Resumo {
        Resumo {
            nome: self.nome,
            nivel: self.nivel,
            xp: self.xp,
        }
    }
}

impl Inimigo {
    fn new(nome: &str, vida: u32, ataque: u32, xp: u32) -> Self {
        Self {
            nome: nome.to_string(),
            vida,
            ataque,
            xp,
        }
    }

    fn esta_vivo(&self) -> bool {
        self.vida > 0
    }

    // Devolve um Dano com o ataque do inimigo.
    fn golpe(&self) -> Dano {
        Dano(self.ataque)
    }

    fn receber(&mut self, dano: Dano) {
        self.vida = self.vida.saturating_sub(dano.0);
    }
}

// Pronta: só escolhe os tipos dos parâmetros (os dois mudam durante a luta).
fn batalha(heroi: &mut Heroi, inimigo: &mut Inimigo) {
    let mut rodadas = 0;
    loop {
        rodadas += 1;
        heroi.atacar(inimigo);
        if !inimigo.esta_vivo() {
            println!(
                "{} venceu {} em {rodadas} rodadas (+{} XP)",
                heroi.nome(),
                inimigo.nome,
                inimigo.xp
            );
            heroi.ganhar_xp(inimigo.xp);
            break;
        }
        heroi.receber(inimigo.golpe());
        if !heroi.esta_vivo() {
            println!("{} caiu diante de {}", heroi.nome(), inimigo.nome);
            break;
        }
    }
}

fn main() {
    let mut frieren = Heroi::new("Frieren", "Maga");
    println!("{}", frieren.status());

    let mut slime = Inimigo::new("Slime", 30, 5, 40);
    // O ataque vem do slime; o resto é novo.
    let mut gigante = Inimigo {
        nome: String::from("Slime Gigante"),
        vida: 60,
        xp: 80,
        ..slime
    };

    batalha(&mut frieren, &mut slime);
    println!("{}", frieren.status());
    batalha(&mut frieren, &mut gigante);
    println!("{}", frieren.status());

    frieren.curar(Cura(15));
    println!("Depois da poção: {}", frieren.status());
    frieren.curar(Cura(50));
    println!("Outra poção:     {}", frieren.status());

    let resumo = frieren.encerrar();
    println!(
        "Fim da jornada: {} terminou no nível {} com {} XP",
        resumo.nome, resumo.nivel, resumo.xp
    );
}
