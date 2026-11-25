//! Executável do `minigrep`: lê a linha de comando e a variável `IGNORE_CASE`, roda a busca e
//! devolve o código de saída do `grep` (0 = achou, 1 = não achou, 2 = erro).
//!
//! A lógica mora na biblioteca (`src/lib.rs`); aqui ficam só o ambiente e os erros.

use std::env;
use std::io;
use std::process::ExitCode;

use minigrep::Config;

const USO: &str = "uso: minigrep [-i | -s] [-n] CONSULTA ARQUIVO";

fn main() -> ExitCode {
    // Só importa se a variável existe, não o valor (Rust Book, 12.5)
    let ignorar_caixa = env::var_os("IGNORE_CASE").is_some();

    let config = match Config::build(env::args(), ignorar_caixa) {
        Ok(config) => config,
        Err(erro) => {
            eprintln!("minigrep: {erro}\n{USO}");
            return ExitCode::from(2);
        }
    };

    match minigrep::executar(&config, &mut io::stdout().lock()) {
        Ok(0) => ExitCode::from(1),
        Ok(_) => ExitCode::SUCCESS,
        Err(erro) => {
            eprintln!("minigrep: {erro}");
            ExitCode::from(2)
        }
    }
}
