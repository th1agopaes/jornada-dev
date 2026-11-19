// Módulo 3 · Aula 2: métodos (impl, &self, &mut self, self) e funções associadas

struct Personagem {
    nome: String,
    vida: u32,
    vida_max: u32,
    nivel: u8,
}

#[derive(Debug)]
struct Lenda {
    nome: String,
    nivel_final: u8,
}

impl Personagem {
    // Função associada (sem self): o "construtor", chamado com ::
    fn new(nome: &str, vida_max: u32) -> Self {
        Self {
            nome: nome.to_string(),
            vida: vida_max,
            vida_max,
            nivel: 1,
        }
    }

    // &self: só lê
    fn nome(&self) -> &str {
        &self.nome
    }

    fn esta_vivo(&self) -> bool {
        self.vida > 0
    }

    fn status(&self) -> String {
        format!(
            "{} (nv. {}) {}/{} HP",
            self.nome, self.nivel, self.vida, self.vida_max
        )
    }

    // &mut self: altera
    fn receber_dano(&mut self, dano: u32) {
        self.vida = self.vida.saturating_sub(dano);
    }

    fn curar(&mut self, pontos: u32) {
        self.vida = (self.vida + pontos).min(self.vida_max);
    }

    fn subir_nivel(&mut self) {
        self.nivel += 1;
        self.vida_max += 10;
        self.vida = self.vida_max;
    }

    // self: consome (o Personagem deixa de existir e vira uma Lenda)
    fn aposentar(self) -> Lenda {
        Lenda {
            nome: self.nome,
            nivel_final: self.nivel,
        }
    }
}

fn main() {
    let mut heroi = Personagem::new("Kael", 100);
    println!("{}", heroi.status());

    heroi.receber_dano(30);
    println!("Depois do golpe: {}", heroi.status());
    heroi.curar(50);
    println!("Depois da poção: {}", heroi.status());
    heroi.subir_nivel();
    println!("Subiu de nível: {}", heroi.status());

    // Método é função com self no primeiro parâmetro: as duas linhas fazem o mesmo
    heroi.receber_dano(200);
    Personagem::receber_dano(&mut heroi, 200);
    println!("{} está vivo? {}", heroi.nome(), heroi.esta_vivo());

    let lenda = heroi.aposentar();
    println!("{lenda:?}");
    println!("{} encerrou no nível {}", lenda.nome, lenda.nivel_final);
}
