# 🟦 MC1 · TypeScript do zero

![TypeScript](https://img.shields.io/badge/TypeScript-7.0-3178C6?logo=typescript&logoColor=white)
![Node.js](https://img.shields.io/badge/Node.js-26-5FA04E?logo=nodedotjs&logoColor=white)
![Vitest](https://img.shields.io/badge/Vitest-5.1-6E9F18?logo=vitest&logoColor=white)
![Biome](https://img.shields.io/badge/Biome-2.6.7-60A5FA?logo=biome&logoColor=white)
![Testes](https://img.shields.io/badge/testes-44%20passando-brightgreen)

Primeiro minicurso da [Jornada Dev](../README.md): TypeScript aprendido do zero, vindo de C, Java e Python, em 7 módulos com exercício de fixação e uma formatura com mini-projeto feito em TDD.

## 🚀 Como rodar

Pré-requisito: Node 26 (roda `.ts` direto, sem compilar).

```bash
npm ci                    # instala as dependências exatas do package-lock.json
npm run validar           # Biome + tsc + Vitest: tudo verde = código 0
node 21-formatura.ts      # demonstração da biblioteca no terminal
```

| Script               | O que faz                                                     |
| -------------------- | ------------------------------------------------------------- |
| `npm run check`      | `tsc`: só verifica os tipos (`noEmit`)                        |
| `npm test`           | `vitest run`: roda os testes uma vez                          |
| `npm run test:watch` | Vitest em modo vigia (roda de novo a cada vez que salva)      |
| `npm run lint`       | `biome check .`: lint + formatação + ordem dos imports        |
| `npm run lint:fix`   | Aplica os consertos seguros do Biome                          |
| `npm run validar`    | `lint && check && test`: o "botão verde" antes de cada commit |

## 📚 Módulos

| #   | Tema                     | Arquivos                                   | Exercício                          | PR                                                     |
| --- | ------------------------ | ------------------------------------------ | ---------------------------------- | ------------------------------------------------------ |
| 1   | Preparação               | `ola.ts`                                   | Função `media`                     | [#1](https://github.com/th1agopaes/jornada-dev/pull/1) |
| 2   | Tipos e variáveis        | `1-variaveis.ts` → `3-any-unknown.ts`      | `4-desafio.ts`: `paraNumero`       | [#2](https://github.com/th1agopaes/jornada-dev/pull/2) |
| 3   | Funções                  | `5-funcoes.ts` → `7-tipos-de-funcao.ts`    | `8-desafio.ts`: `aplicarBonus`     | [#3](https://github.com/th1agopaes/jornada-dev/pull/3) |
| 4   | Objetos e coleções       | `9-objetos.ts` → `11-uniao.ts`             | `12-desafio.ts`: boletim           | [#4](https://github.com/th1agopaes/jornada-dev/pull/4) |
| 5   | Classes e módulos        | `13-classes.ts`, `14-modulos.ts`, `banco/` | `15-desafio.ts` + `escola/`: turma | [#5](https://github.com/th1agopaes/jornada-dev/pull/5) |
| 6   | Erros, nulos e genéricos | `16-erros.ts` → `19-genericos.ts`          | `20-desafio.ts` + `repo/`          | [#6](https://github.com/th1agopaes/jornada-dev/pull/6) |
| 7   | Testes e qualidade       | `*.test.ts`, `biome.json`                  | 21 testes em `repo/` e `escola/`   | [#7](https://github.com/th1agopaes/jornada-dev/pull/7) |
| 🎓  | Formatura                | `biblioteca/`, `21-formatura.ts`           | Biblioteca em memória com TDD      | [#8](https://github.com/th1agopaes/jornada-dev/pull/8) |

## 🎓 Formatura: biblioteca em memória

Cadastro de livros e leitores, empréstimo e devolução com limite por leitor, feito com testes antes do código (TDD). Detalhes e decisões no [README da biblioteca](biblioteca/README.md).

- `Livro`: classe com estado privado (`#situacao`) modelado como **união discriminada**.
- `Biblioteca`: coordena dois `Repositorio<T>` genéricos e confere tudo **antes** de mudar qualquer estado.
- Erros próprios para cada regra violada; **44 testes** no total.

## 🧠 O que aprendi

- **TypeScript = JavaScript + tipos que somem na execução.** O Node roda o arquivo; quem confere os tipos é o `tsc`.
- **Tipagem estrutural:** vale o formato do objeto, não o nome da classe (no Java é nominal).
- **Uniões discriminadas + narrowing** transformam estados impossíveis em erro de tipo.
- **`#campo`** é privado de verdade; `private` é só aviso do `tsc`.
- **Erros:** `catch` recebe `unknown` (não existe `throws`); caso esperado → devolver valor, impossível continuar → `throw`.
- **Genéricos** com restrição (`Repositorio<T extends ComId>`) e o padrão repositório (`buscar` × `obter`).
- **Testes:** AAA, `toBe` × `toEqual`, `toThrow` com arrow, tabelas com `it.each`, valor-limite e TDD (vermelho → verde).
- **Cinco revisores:** `node` (sintaxe e execução), `tsc` (tipos), Biome (armadilhas e formato), Vitest (comportamento) e eu, vendo o teste falhar uma vez.

<details>
<summary>📌 Consulta rápida</summary>

| Recurso            | Em uma linha                                                                |
| ------------------ | --------------------------------------------------------------------------- |
| `map`              | Transforma cada item; array novo do mesmo tamanho                           |
| `filter`           | Mantém só os itens que passam no teste                                      |
| `reduce`           | Junta tudo num valor só, com acumulador                                     |
| `find`             | Primeiro item que passa, ou `undefined`                                     |
| `[...lista]`       | Cópia rasa: array novo, mesmos objetos dentro                               |
| `unknown`          | Aceita tudo, mas obriga a conferir antes de usar                            |
| União discriminada | Objetos com campo etiqueta (`tipo`); o `switch` libera os campos de cada um |
| `?.`               | Para no primeiro `null`/`undefined` e devolve `undefined`                   |
| `??`               | Troca só `null`/`undefined` (o `\|\|` troca qualquer falso)                 |
| `import type`      | Importa algo que só existe para o `tsc`                                     |
| `T extends ComId`  | Genérico que exige um formato mínimo                                        |
| `toBe` × `toEqual` | Mesma coisa × mesmo conteúdo                                                |

</details>

## 🔗 Referências

- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/intro.html)
- [TypeScript direto no Node](https://nodejs.org/learn/typescript/run-natively)
- [Vitest](https://vitest.dev/guide/) · [Biome](https://biomejs.dev/guides/getting-started/)
