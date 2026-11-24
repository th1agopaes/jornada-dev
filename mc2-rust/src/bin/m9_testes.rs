//! MC2 · Rust do zero — Módulo 9 · Aula 1: anatomia de um teste
//! Objetivo: escrever testes com `#[test]`, `assert!` e `assert_eq!` e rodá-los com `cargo test`.
//! Rodar:    `cargo run --bin m9_testes`
//! Testar:   `cargo test --bin m9_testes`

// --- 1. O código que vamos testar ---------------------------------

const VIDA_MAX: u32 = 100;

// `PartialEq` deixa o teste comparar o herói inteiro; `Debug` imprime os dois lados se falhar
#[derive(Debug, PartialEq)]
struct Heroi {
    nome: String,
    vida: u32,
}

impl Heroi {
    fn new(nome: &str) -> Self {
        Self {
            nome: nome.to_string(),
            vida: VIDA_MAX,
        }
    }

    fn receber_dano(&mut self, dano: u32) {
        self.vida = self.vida.saturating_sub(dano);
    }

    fn curar(&mut self, cura: u32) {
        self.vida = self.vida.saturating_add(cura).min(VIDA_MAX);
    }

    fn esta_vivo(&self) -> bool {
        self.vida > 0
    }
}

// Dano depois da defesa; nunca menos que 1 (todo golpe arranha)
fn dano_final(ataque: u32, defesa: u32) -> u32 {
    ataque.saturating_sub(defesa).max(1)
}

fn com_critico(dano: u32, critico: bool) -> u32 {
    if critico { dano * 2 } else { dano }
}

// --- 2. O programa de sempre ---------------------------------

fn main() {
    let mut heroi = Heroi::new("Frieren");
    let dano = com_critico(dano_final(40, 15), true);
    heroi.receber_dano(dano);
    println!("{} levou {dano} e ficou com {}", heroi.nome, heroi.vida);
    // -> Frieren levou 50 e ficou com 50

    heroi.curar(80);
    let vivo = heroi.esta_vivo();
    println!("Depois da cura: {} (vivo? {vivo})", heroi.vida);
    // -> Depois da cura: 100 (vivo? true)
}

// --- 3. O módulo de testes ---------------------------------

// `cfg(test)`: o módulo só existe no `cargo test`; o `cargo run` nem compila esta parte
#[cfg(test)]
mod tests {
    // Módulo filho enxerga o privado do pai (Módulo 6); `super::*` traz todos os nomes
    use super::*;

    #[test]
    fn defesa_reduz_o_dano() {
        assert_eq!(dano_final(30, 10), 20);
    }

    #[test]
    fn todo_golpe_tira_ao_menos_1() {
        assert_eq!(dano_final(5, 50), 1); // sem o `max(1)`, seria 0
    }

    #[test]
    fn critico_dobra_e_normal_nao_muda() {
        assert_eq!(com_critico(25, true), 50);
        assert_eq!(com_critico(25, false), 25);
    }

    // --- 4. `assert!`, `assert_ne!` e mensagens ---------------------------------

    #[test]
    fn heroi_novo_esta_vivo() {
        let heroi = Heroi::new("Fern");
        // Mensagem extra: argumentos depois da condição, como num `format!`
        let Heroi { nome, vida } = &heroi;
        assert!(heroi.esta_vivo(), "{nome} nasceu com {vida} de vida");
    }

    #[test]
    fn critico_muda_o_dano() {
        assert_ne!(com_critico(10, true), com_critico(10, false));
    }

    // --- 5. Bordas e a struct inteira ---------------------------------

    #[test]
    fn dano_maior_que_a_vida_zera_sem_panic() {
        // Preparar, agir, conferir (Arrange, Act, Assert)
        let mut heroi = Heroi::new("Stark");
        heroi.receber_dano(250);
        assert_eq!(heroi.vida, 0);
        assert!(!heroi.esta_vivo());
    }

    #[test]
    fn cura_nao_passa_da_vida_maxima() {
        let mut heroi = Heroi::new("Frieren");
        heroi.receber_dano(30);
        heroi.curar(500);
        // Compara a struct inteira: nome e vida de uma vez (pede `PartialEq` + `Debug`)
        let esperado = Heroi {
            nome: "Frieren".to_string(),
            vida: VIDA_MAX,
        };
        assert_eq!(heroi, esperado);
    }
}
