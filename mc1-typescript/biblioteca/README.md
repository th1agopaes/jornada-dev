# Biblioteca em memória · formatura do MC1

Cadastro de livros e leitores, empréstimo e devolução com limite por leitor.
TypeScript puro, sem banco: tudo em memória. Feito com TDD (teste antes do código).

## Como rodar

Na pasta `mc1-typescript`:

| Comando | O que faz |
| --- | --- |
| `npm run validar` | Biome + tsc + Vitest (o "botão verde") |
| `node 21-formatura.ts` | Demonstração no terminal |

## Estrutura

| Arquivo | Papel |
| --- | --- |
| `erros.ts` | Define as classes de erro de negócio do domínio (`LivroIndisponivelError`, `DevolucaoInvalidaError`, `LimiteDeEmprestimosError`). |
| `livro.ts` | Modela a entidade `Livro` com estado encapsulado (`#situacao`), o tipo `Situacao` e suas regras próprias de empréstimo e devolução. |
| `leitor.ts` | Define o contrato de dados (`interface Leitor`) com `id` e `nome` para representar quem pega livros emprestados. |
| `biblioteca.ts` | Coordena o acervo, os leitores, as pesquisas e as regras cruzadas entre eles (usa o `Repositorio<T>` de `../repo`). |

## Decisões

- **`Livro` é classe, `Leitor` é `interface`:** O `Livro` possui estado mutável privado (`#situacao`) e regras de negócio que protegem suas transições (`emprestar` e `devolver`). Já o `Leitor` é apenas uma estrutura simples de dados sem comportamento próprio, bastando uma `interface` leve.
- **Situação como união discriminada:** Garante em tempo de compilação que o `leitorId` só exista quando `tipo` for `"emprestado"`. Isso torna estados impossíveis (como um livro `"disponivel"` com um leitor associado) erros de tipo no TypeScript.
- **Limite calculado, não guardado:** Evita duplicação de estado e bugs de dessincronização. Se houvesse um contador manual no leitor, uma falha ao incrementar ou decrementar deixaria os dados inconsistentes; calcular via `emprestadosPor(leitorId).length` garante que a fonte da verdade seja sempre o estado atual dos livros.
- **`emprestar` confere tudo antes de mudar:** Aplica o princípio da atomicidade. Todas as pré-condições (existência do leitor, existência do livro, limite de empréstimos e disponibilidade) são validadas antes de qualquer mutação, garantindo que uma operação inválida nunca deixe o sistema alterado pela metade.
- **Erro lançado em vez de `boolean`:** Retornar `false` apenas diz que algo falhou, mas esconde o motivo. Lançar erros tipados comunica exatamente qual regra foi violada (livro inexistente, indisponível ou limite atingido) e obriga quem chama a tratar a falha de forma explícita.
