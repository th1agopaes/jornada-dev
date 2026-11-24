//! MC2 · Rust do zero — Módulo 9 · Aula 3: biblioteca, testes de integração e doc tests
//! Objetivo: pôr o código num crate de biblioteca e testá-lo por dentro, por fora e pela doc.
//! Testar:   `cargo test --lib`, `cargo test --test dados` e `cargo test --doc`
//!
//! Rolagens de dados na notação de RPG: `2d6+3` são dois dados de 6 faces, mais 3.

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

use rand::{Rng, RngExt};

// --- 1. A rolagem ---------------------------------

/// Uma rolagem na notação `NdF+B`: `N` dados de `F` faces, mais um bônus `B`.
///
/// Os campos são `u8`, então um número acima de 255 já é recusado na leitura.
#[derive(Debug, PartialEq)]
pub struct Rolagem {
    quantidade: u8,
    faces: u8,
    bonus: u8,
}

// --- 2. Os erros ---------------------------------

/// O que pode dar errado ao ler uma rolagem.
#[derive(Debug, PartialEq)]
pub enum ErroRolagem {
    /// Faltou o `d` (`"2x6"`); guarda o texto lido.
    Formato(String),
    /// Quantidade ou faces iguais a 0 (`"0d6"`, `"2d0"`).
    Zero,
    /// Número inválido ou acima de 255 (`"2dx"`, `"300d6"`).
    Numero(ParseIntError),
}

impl fmt::Display for ErroRolagem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErroRolagem::Formato(texto) => write!(f, "falta o `d` em \"{texto}\""),
            ErroRolagem::Zero => write!(f, "quantidade e faces precisam ser maiores que 0"),
            ErroRolagem::Numero(erro) => write!(f, "número inválido ({erro})"),
        }
    }
}

// Com `Error`, o `?` do `main` converte para `Box<dyn Error>` (o `main` do *minigrep*)
impl Error for ErroRolagem {}

impl From<ParseIntError> for ErroRolagem {
    fn from(erro: ParseIntError) -> Self {
        ErroRolagem::Numero(erro)
    }
}

// --- 3. Lendo a notação ---------------------------------

impl Rolagem {
    /// Lê uma rolagem como `"2d6+3"` ou `"1d20"` (sem bônus).
    ///
    /// # Errors
    ///
    /// Devolve [`ErroRolagem`] se faltar o `d`, se a quantidade ou as faces forem 0
    /// ou se um número não couber em `u8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use mc2_rust::{ErroRolagem, Rolagem};
    ///
    /// let rolagem = Rolagem::ler("2d6+3")?;
    /// assert_eq!(rolagem.maximo(), 15);
    /// assert_eq!(Rolagem::ler("2d0"), Err(ErroRolagem::Zero));
    /// # Ok::<(), ErroRolagem>(())
    /// ```
    pub fn ler(texto: &str) -> Result<Self, ErroRolagem> {
        let (quantidade, resto) = texto
            .split_once('d')
            .ok_or_else(|| ErroRolagem::Formato(texto.to_string()))?;
        let (faces, bonus) = resto.split_once('+').unwrap_or((resto, "0")); // sem `+`: bônus 0
        Ok(Self {
            quantidade: positivo(quantidade)?,
            faces: positivo(faces)?,
            bonus: bonus.parse::<u8>()?,
        })
    }
}

// Privada: fica fora da API; só os testes unitários deste arquivo a enxergam
fn positivo(texto: &str) -> Result<u8, ErroRolagem> {
    let numero = texto.parse::<u8>()?;
    if numero == 0 {
        return Err(ErroRolagem::Zero);
    }
    Ok(numero)
}

// --- 4. Rolando ---------------------------------

impl Rolagem {
    /// O menor total possível: todos os dados em 1.
    pub fn minimo(&self) -> u32 {
        u32::from(self.quantidade) + u32::from(self.bonus)
    }

    /// O maior total possível: todos os dados na face mais alta.
    pub fn maximo(&self) -> u32 {
        u32::from(self.quantidade) * u32::from(self.faces) + u32::from(self.bonus)
    }

    /// Rola os dados com o gerador recebido: com semente nos testes, o do sistema no jogo.
    ///
    /// # Examples
    ///
    /// ```
    /// use mc2_rust::Rolagem;
    /// use rand::SeedableRng;
    /// use rand::rngs::ChaCha8Rng;
    ///
    /// let mut rng = ChaCha8Rng::seed_from_u64(7);
    /// let rolagem = Rolagem::ler("3d8").unwrap();
    /// let total = rolagem.rolar(&mut rng);
    /// assert!((rolagem.minimo()..=rolagem.maximo()).contains(&total));
    /// ```
    pub fn rolar(&self, rng: &mut impl Rng) -> u32 {
        let faces = u32::from(self.faces);
        let dados: u32 = (0..self.quantidade)
            .map(|_| rng.random_range(1..=faces))
            .sum();
        dados + u32::from(self.bonus)
    }
}

// --- 5. Mostrando ---------------------------------

// "2d6+3"; sem bônus, só "1d20": o mesmo texto que o `ler` aceita
impl fmt::Display for Rolagem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Rolagem {
            quantidade,
            faces,
            bonus,
        } = self;
        if *bonus == 0 {
            write!(f, "{quantidade}d{faces}")
        } else {
            write!(f, "{quantidade}d{faces}+{bonus}")
        }
    }
}

// --- 6. Testes unitários ---------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // Função privada: o teste unitário alcança; o de integração (`tests/`), não
    #[test]
    fn positivo_aceita_de_1_a_255() {
        assert_eq!(positivo("1"), Ok(1));
        assert_eq!(positivo("255"), Ok(255));
    }

    #[test]
    fn positivo_recusa_zero_e_estouro() {
        assert_eq!(positivo("0"), Err(ErroRolagem::Zero));
        assert!(matches!(positivo("256"), Err(ErroRolagem::Numero(_))));
    }

    // Campos privados também: aqui dentro dá para montar a struct direto
    #[test]
    fn le_sem_bonus() {
        let esperado = Rolagem {
            quantidade: 1,
            faces: 20,
            bonus: 0,
        };
        assert_eq!(Rolagem::ler("1d20"), Ok(esperado));
    }
}
