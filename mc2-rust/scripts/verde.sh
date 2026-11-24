#!/usr/bin/env bash
# MC2 · Rust do zero — Módulo 9 · Aula 4: o botão verde
# Objetivo: formatação, Clippy e testes num comando só; para no primeiro passo que falhar.
# Rodar:    ./scripts/verde.sh (de qualquer pasta)

set -euo pipefail # erro, variável indefinida ou falha no meio de um pipe: para tudo

cd "$(dirname "$0")/.." # roda sempre da pasta mc2-rust, onde está o Cargo.toml

echo "1/3 · formatação (cargo fmt --check)"
cargo fmt --check

echo "2/3 · Clippy em tudo, aviso vira erro (cargo clippy --all-targets -- -D warnings)"
cargo clippy --all-targets --quiet -- -D warnings

echo "3/3 · testes (cargo test)"
cargo test --quiet

echo "✔ verde"
