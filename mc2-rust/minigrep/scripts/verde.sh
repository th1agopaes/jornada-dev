#!/usr/bin/env bash
# Botão verde do minigrep: formatação, Clippy e testes; para no primeiro passo que falhar.
# Uso: ./scripts/verde.sh (de qualquer pasta)

set -euo pipefail

cd "$(dirname "$0")/.." # a pasta do Cargo.toml do minigrep

echo "1/3 · formatação (cargo fmt --check)"
cargo fmt --check

echo "2/3 · Clippy em tudo, aviso vira erro (cargo clippy --all-targets -- -D warnings)"
cargo clippy --all-targets --quiet -- -D warnings

echo "3/3 · testes (cargo test)"
cargo test --quiet

echo "✔ verde"
