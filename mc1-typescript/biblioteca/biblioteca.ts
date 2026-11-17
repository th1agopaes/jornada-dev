import { Repositorio } from "../repo/repositorio.ts";
import { LimiteDeEmprestimosError } from "./erros.ts";
import type { Leitor } from "./leitor.ts";
import type { Livro } from "./livro.ts";

export class Biblioteca {
  readonly limitePorLeitor: number;
  #livros = new Repositorio<Livro>();
  #leitores = new Repositorio<Leitor>();

  constructor(limitePorLeitor = 3) {
    this.limitePorLeitor = limitePorLeitor;
  }

  cadastrarLivro(livro: Livro): void {
    this.#livros.adicionar(livro);
  }

  cadastrarLeitor(leitor: Leitor): void {
    this.#leitores.adicionar(leitor);
  }

  acervo(): Livro[] {
    return this.#livros.listar();
  }

  disponiveis(): Livro[] {
    return this.acervo().filter((l) => l.disponivel);
  }

  emprestadosPor(leitorId: number): Livro[] {
    return this.acervo().filter(
      (l) =>
        l.situacao.tipo === "emprestado" && l.situacao.leitorId === leitorId,
    );
  }

  pesquisar(termo: string): Livro[] {
    const t = termo.trim().toLowerCase();
    if (t === "") {
      return [];
    }
    return this.acervo().filter(
      (l) =>
        l.titulo.toLowerCase().includes(t) || l.autor.toLowerCase().includes(t),
    );
  }

  emprestar(livroId: number, leitorId: number): void {
    this.#leitores.obter(leitorId);
    const livro = this.#livros.obter(livroId);

    if (this.emprestadosPor(leitorId).length >= this.limitePorLeitor) {
      throw new LimiteDeEmprestimosError(leitorId, this.limitePorLeitor);
    }

    livro.emprestar(leitorId);
  }

  devolver(livroId: number): void {
    this.#livros.obter(livroId).devolver();
  }
}
