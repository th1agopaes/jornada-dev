# 🦀 MC2 · Rust do zero

![Rust](https://img.shields.io/badge/Rust-1.100-CE422B?logo=rust&logoColor=white)
![Edição](https://img.shields.io/badge/edi%C3%A7%C3%A3o-2024-CE422B)
![rand](https://img.shields.io/badge/rand-0.10.4-blue)
![Clippy](https://img.shields.io/badge/Clippy-0%20avisos-brightgreen)
![Testes](https://img.shields.io/badge/testes-42%20%2B%2020%20passando-brightgreen)

Segundo minicurso da [Jornada Dev](../README.md): Rust aprendido do zero, vindo de C, Java, Python e TypeScript, em 10 módulos com aulas e exercício de fixação, mais uma formatura com mini-projeto. Base: [The Rust Book](https://doc.rust-lang.org/book/). É o preparo para o próximo projeto: uma engine 2D e um jogo, do zero.

## 🚀 Como rodar

Pré-requisito: Rust estável com Cargo ([rustup](https://rustup.rs/)).

```bash
cd mc2-rust
cargo run --bin m1_variaveis          # cada arquivo de src/bin/ é um executável
cargo run --bin m7_desafio            # exercícios: mN_desafio
./scripts/verde.sh                    # o botão verde: fmt + Clippy + testes

cd minigrep                           # a formatura é um pacote à parte
cargo run -q -- -n fogo exemplos/bestiario.txt
./scripts/verde.sh
```

| Comando                                     | O que faz                                                    |
| ------------------------------------------- | ------------------------------------------------------------ |
| `cargo run --bin <nome>`                    | Compila e roda um executável de `src/bin/`                   |
| `cargo test`                                | Testes unitários, de integração (`tests/`) e de documentação |
| `cargo clippy --all-targets -- -D warnings` | Clippy em tudo, inclusive nos testes; aviso vira erro        |
| `cargo fmt --check`                         | Confere a formatação do rustfmt sem mudar nada               |
| `./scripts/verde.sh`                        | Os três acima, em ordem; para no primeiro que falhar         |

## 🗂️ Estrutura

```text
mc2-rust/
├── Cargo.toml        # um pacote só: [lints], dependência rand (feature chacha)
├── rustfmt.toml
├── scripts/verde.sh
├── src/bin/          # aulas e exercícios: mN_<tema>.rs (pasta com main.rs quando tem módulos)
├── src/lib.rs        # biblioteca mc2_rust (rolagens de dados), do Módulo 9
├── tests/dados.rs    # testes de integração da biblioteca
└── minigrep/         # 🎓 formatura: pacote Cargo próprio
```

## 📚 Módulos

| #   | Tema                               | Aulas (`src/bin/`)                                              | Exercício                                    | PR                                                       |
| --- | ---------------------------------- | --------------------------------------------------------------- | -------------------------------------------- | -------------------------------------------------------- |
| 0   | Preparação                         | rustup, rustc e Cargo; "Hello, world!" no sandbox               | —                                            | —                                                        |
| 1   | Variáveis, tipos, funções e fluxo  | `m1_variaveis`, `m1_tipos`, `m1_funcoes`, `m1_fluxo`            | `m1_desafio`: boletim + Fibonacci            | [#10](https://github.com/th1agopaes/jornada-dev/pull/10) |
| 2   | Ownership, referências e borrowing | `m2_ownership`, `m2_borrowing`, `m2_slices`                     | `m2_desafio`: analisador de anotações        | [#11](https://github.com/th1agopaes/jornada-dev/pull/11) |
| 3   | Structs e métodos                  | `m3_structs`, `m3_metodos`                                      | `m3_desafio`: arena de RPG                   | [#12](https://github.com/th1agopaes/jornada-dev/pull/12) |
| 4   | Enums, `match` e `Option`          | `m4_enums`, `m4_option`                                         | `m4_desafio`: masmorra                       | [#13](https://github.com/th1agopaes/jornada-dev/pull/13) |
| 5   | Coleções e erros (`Result`, `?`)   | `m5_vec`, `m5_string_hashmap`, `m5_result`                      | `m5_desafio`: inventário                     | [#14](https://github.com/th1agopaes/jornada-dev/pull/14) |
| 6   | Módulos, crates e dependências     | `m6_modulos`, `m6_masmorra/`, `m6_rand`                         | `m6_desafio/`: inventário em módulos + loot  | [#15](https://github.com/th1agopaes/jornada-dev/pull/15) |
| 7   | Traits, genéricos e lifetimes      | `m7_traits`, `m7_genericos`, `m7_traits_std`, `m7_lifetimes`    | `m7_desafio`: torneio da guilda              | [#16](https://github.com/th1agopaes/jornada-dev/pull/16) |
| 8   | Closures e iteradores              | `m8_closures`, `m8_iteradores`, `m8_adaptadores`                | `m8_desafio`: mural de missões               | [#17](https://github.com/th1agopaes/jornada-dev/pull/17) |
| 9   | Testes, Clippy e rustfmt           | `m9_testes`, `m9_testes_jogo`, `m9_dados` + `lib.rs` + `tests/` | 15 testes no `m8_desafio`                    | [#18](https://github.com/th1agopaes/jornada-dev/pull/18) |
| 🎓  | Formatura                          | `minigrep/`                                                     | `grep` pequeno com testes desde o 1º arquivo | [#19](https://github.com/th1agopaes/jornada-dev/pull/19) |

## 🎓 Formatura: minigrep

Um `grep` pequeno, do projeto do Rust Book ([cap. 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) e [13.3](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html)), que junta o minicurso inteiro. Detalhes e decisões no [README do minigrep](minigrep/README.md).

- `buscar` devolve uma **esteira preguiçosa** (`impl Iterator`) de ocorrências que **emprestam** do texto (`'a`), sem copiar nenhuma linha.
- `Config::build` lê `-i`, `-s` e `-n` de qualquer iterador de `String`, com erros em um `enum` com `Display` + `Error`.
- O ambiente e a saída são **injetados** (`bool` e `&mut impl Write`), então tudo é testável sem `unsafe`.
- Códigos de saída do `grep` (0 achou, 1 não achou, 2 erro), erros no stderr e **20 testes** (unitários, de ponta a ponta e de documentação).

## 🧠 O que aprendi

- **Ownership:** todo valor tem um dono; quando o dono sai do escopo, o compilador insere o `drop`. Sem garbage collector e sem `free` à mão.
- **Borrowing:** várias `&` **ou** uma `&mut`, e a referência nunca vive mais que o dono. A assinatura conta a história: só lê → `&T`; altera → `&mut T`; guarda ou consome → `T`.
- **Tipos fazem o trabalho:** `Option` no lugar do `null`, `Result` + `?` no lugar das exceções, `enum` + `match` exaustivo para estados.
- **Traits** são contratos (sem herança); **genéricos** com limites viram código específico na compilação (monomorfização), e `dyn Trait` faz o despacho dinâmico.
- **Lifetimes** descrevem de onde vem uma referência devolvida; quase sempre a elisão resolve sozinha.
- **Closures e iteradores** são preguiçosos e de custo zero: a esteira só anda quando alguém puxa.
- **Qualidade:** testes no mesmo arquivo, em `tests/` e na documentação; Clippy com `--all-targets` e `-D warnings`; rustfmt; tudo num botão verde antes de cada commit.
- **Os revisores:** `rustc` (tipos e empréstimos), Clippy (armadilhas e estilo), rustfmt (formato), `cargo test` (comportamento) e eu, comparando a saída com a esperada (erro dentro de texto compila calado).

<details>
<summary>📌 Consulta rápida</summary>

| Recurso                             | Em uma linha                                                          |
| ----------------------------------- | --------------------------------------------------------------------- |
| `let` × `let mut`                   | Imutável por padrão; `mut` libera a troca                             |
| `String` × `&str`                   | Dona do texto no heap × fatia emprestada                              |
| `.clone()`                          | Cópia explícita (e cara) do que está no heap                          |
| `Option<T>`                         | `Some(x)` ou `None`: substitui o `null`                               |
| `Result<T, E>` + `?`                | `Ok` ou `Err`; o `?` devolve o erro na hora (convertendo pelo `From`) |
| `match`                             | Exaustivo, sem fall-through, é expressão                              |
| `if let` / `let ... else`           | `match` de um braço só / cláusula de guarda                           |
| `impl Trait for Tipo`               | Cumpre um contrato; os métodos padrão vêm de graça                    |
| `T: Limite` / `impl Trait`          | Genérico com limite de trait / "qualquer tipo que cumpra"             |
| `'a`                                | Liga a referência devolvida à entrada de onde ela vem                 |
| `Fn` / `FnMut` / `FnOnce`           | Closure que só lê / que altera / que consome o que capturou           |
| `iter` / `iter_mut` / `into_iter`   | Esteira de `&T` / `&mut T` / `T`                                      |
| `map`, `filter`, `enumerate`, `zip` | Adaptadores preguiçosos: só rodam quando um consumidor puxa           |
| `collect`                           | Junta a esteira num `Vec`, `String`, `HashMap` ou `Result<Vec<_>, _>` |
| `Box<dyn Trait>`                    | Dono no heap de "algum tipo que cumpre o trait"                       |
| `#[cfg(test)] mod tests`            | Testes no mesmo arquivo, com acesso ao privado (`use super::*`)       |
| `assert_eq!`                        | Compara e mostra `left`/`right` na falha                              |
| `tests/*.rs`                        | Testes de integração: só enxergam a API pública                       |
| `cargo clippy --all-targets`        | Sem o `--all-targets`, o Clippy nunca olha os testes                  |

</details>

## 🔗 Referências

- [The Rust Book](https://doc.rust-lang.org/book/) · [The Cargo Book](https://doc.rust-lang.org/cargo/) · [Edition Guide](https://doc.rust-lang.org/edition-guide/)
- [Biblioteca padrão](https://doc.rust-lang.org/std/) · [Clippy](https://doc.rust-lang.org/clippy/) · [rustfmt](https://rust-lang.github.io/rustfmt/)
- [crates.io](https://crates.io/) · [docs.rs](https://docs.rs/) · [rand](https://docs.rs/rand/)
