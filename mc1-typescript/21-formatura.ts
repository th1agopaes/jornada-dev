import { Biblioteca } from "./biblioteca/biblioteca.ts";
import {
  DevolucaoInvalidaError,
  LimiteDeEmprestimosError,
  LivroIndisponivelError,
} from "./biblioteca/erros.ts";
import { descreverSituacao, Livro } from "./biblioteca/livro.ts";
import { NaoEncontradoError } from "./repo/erros.ts";

const bib = new Biblioteca(2);
bib.cadastrarLivro(new Livro(1, "Dom Casmurro", "Machado de Assis"));
bib.cadastrarLivro(
  new Livro(2, "Memórias Póstumas de Brás Cubas", "Machado de Assis"),
);
bib.cadastrarLivro(new Livro(3, "Vidas Secas", "Graciliano Ramos"));
bib.cadastrarLeitor({ id: 10, nome: "Haru" });
bib.cadastrarLeitor({ id: 20, nome: "Bia" });

function executar(descricao: string, acao: () => void): void {
  try {
    acao();
    console.log(`✅ ${descricao}`);
  } catch (erro) {
    if (
      erro instanceof DevolucaoInvalidaError ||
      erro instanceof LimiteDeEmprestimosError ||
      erro instanceof LivroIndisponivelError ||
      erro instanceof NaoEncontradoError
    ) {
      console.log(`✖ ${descricao} → ${erro.name}: ${erro.message}`);
    } else {
      throw erro;
    }
  }
}

executar("Haru pega o livro 1", () => bib.emprestar(1, 10));
executar("Bia tenta pegar o livro 1", () => bib.emprestar(1, 20));
executar("Haru pega o livro 2", () => bib.emprestar(2, 10));
executar("Haru tenta pegar o livro 3", () => bib.emprestar(3, 10));
executar("devolver o livro 3", () => bib.devolver(3));
executar("leitor 99 pega o livro 3", () => bib.emprestar(3, 99));
executar("Haru devolve o livro 1", () => bib.devolver(1));

console.log("\nAcervo:");
for (const livro of bib.acervo()) {
  console.log(`- ${livro.titulo}: ${descreverSituacao(livro.situacao)}`);
}
