//! Testes de ponta a ponta: rodam o executável de verdade, como no terminal, e conferem a
//! saída, os erros e o código de saída.

use std::process::{Command, Output};

const BESTIARIO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/exemplos/bestiario.txt");

/// Roda o `minigrep` com `args` e só as variáveis de `ambiente`: o `IGNORE_CASE` de quem roda
/// os testes não vaza para dentro deles.
fn minigrep(args: &[&str], ambiente: &[(&str, &str)]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_minigrep"))
        .args(args)
        .env_remove("IGNORE_CASE")
        .envs(ambiente.iter().copied())
        .output()
        .expect("o executável deveria rodar")
}

fn texto(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("saída em UTF-8")
}

#[test]
fn achou_imprime_as_linhas_e_sai_com_0() {
    let saida = minigrep(&["-n", "fogo", BESTIARIO], &[]);

    assert_eq!(saida.status.code(), Some(0));
    let esperado = "\
3:Slime: fraco contra fogo; aparece em bandos perto da água.
5:Dragão Vermelho: imune a fogo; foge quando perde metade da vida.
";
    assert_eq!(texto(&saida.stdout), esperado);
    assert_eq!(texto(&saida.stderr), "");
}

#[test]
fn nao_achou_sai_com_1_sem_imprimir_nada() {
    let saida = minigrep(&["mímico", BESTIARIO], &[]);

    assert_eq!(saida.status.code(), Some(1));
    assert_eq!(texto(&saida.stdout), "");
    assert_eq!(texto(&saida.stderr), "");
}

#[test]
fn erro_vai_para_o_stderr_e_sai_com_2() {
    let saida = minigrep(&["fogo"], &[]);

    assert_eq!(saida.status.code(), Some(2));
    assert_eq!(texto(&saida.stdout), "");
    let esperado = "minigrep: faltou o arquivo\nuso: minigrep [-i | -s] [-n] CONSULTA ARQUIVO\n";
    assert_eq!(texto(&saida.stderr), esperado);
}

#[test]
fn ignore_case_liga_e_a_opcao_s_desliga() {
    let com_variavel = minigrep(&["mímico", BESTIARIO], &[("IGNORE_CASE", "1")]);
    assert_eq!(texto(&com_variavel.stdout).lines().count(), 2);

    let opcao_ganha = minigrep(&["-s", "mímico", BESTIARIO], &[("IGNORE_CASE", "1")]);
    assert_eq!(opcao_ganha.status.code(), Some(1));
}
