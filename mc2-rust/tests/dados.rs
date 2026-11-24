//! MC2 · Rust do zero — Módulo 9 · Aula 3: testes de integração da biblioteca
//! Objetivo: testar o crate `mc2_rust` por fora, só com o que é `pub`.
//! Testar:   `cargo test --test dados`

use rand::SeedableRng;
use rand::rngs::ChaCha8Rng;

use mc2_rust::{ErroRolagem, Rolagem};

// --- 1. Ler e escrever ---------------------------------

// Ida e volta: o `Display` devolve o mesmo texto que o `ler` aceitou
#[test]
fn le_e_escreve_a_mesma_notacao() -> Result<(), ErroRolagem> {
    for texto in ["1d20", "2d6+3", "4d6", "10d10+255"] {
        assert_eq!(Rolagem::ler(texto)?.to_string(), texto);
    }
    Ok(())
}

#[test]
fn recusa_notacoes_invalidas() {
    let esperado = Err(ErroRolagem::Formato("2x6".to_string()));
    assert_eq!(Rolagem::ler("2x6"), esperado);
    assert_eq!(Rolagem::ler("0d6"), Err(ErroRolagem::Zero));
    for texto in ["300d6", "2dx", "2d6+", "d6"] {
        let recusou = matches!(Rolagem::ler(texto), Err(ErroRolagem::Numero(_)));
        assert!(recusou, "{texto} passou");
    }
}

// --- 2. Rolar ---------------------------------

#[test]
fn rolagem_fica_entre_minimo_e_maximo() -> Result<(), ErroRolagem> {
    let rolagem = Rolagem::ler("3d6+2")?;
    let faixa = rolagem.minimo()..=rolagem.maximo(); // 5..=20
    let mut rng = ChaCha8Rng::seed_from_u64(99);
    for _ in 0..1000 {
        let total = rolagem.rolar(&mut rng);
        assert!(faixa.contains(&total), "saiu {total}");
    }
    Ok(())
}

// Dado de 1 face: o sorteio não tem escolha, então o total é conhecido
#[test]
fn dado_de_uma_face_sempre_da_o_minimo() -> Result<(), ErroRolagem> {
    let rolagem = Rolagem::ler("5d1+1")?;
    let mut rng = ChaCha8Rng::seed_from_u64(1);
    assert_eq!(rolagem.rolar(&mut rng), 6);
    Ok(())
}
