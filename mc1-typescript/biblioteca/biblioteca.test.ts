import { beforeEach, describe, expect, it } from "vitest";
import { IdDuplicadoError, NaoEncontradoError } from "../repo/erros.ts";
import { Biblioteca } from "./biblioteca.ts";
import { LimiteDeEmprestimosError, LivroIndisponivelError } from "./erros.ts";
import { Livro } from "./livro.ts";

describe("Biblioteca", () => {
  let bib: Biblioteca;

  beforeEach(() => {
    bib = new Biblioteca(2);
    bib.cadastrarLivro(new Livro(1, "Dom Casmurro", "Machado de Assis"));
    bib.cadastrarLivro(
      new Livro(2, "Memórias Póstumas de Brás Cubas", "Machado de Assis"),
    );
    bib.cadastrarLivro(new Livro(3, "Vidas Secas", "Graciliano Ramos"));
    bib.cadastrarLeitor({ id: 10, nome: "Haru" });
    bib.cadastrarLeitor({ id: 20, nome: "Bia" });
  });

  it("usa 3 como limite padrão", () => {
    expect(new Biblioteca().limitePorLeitor).toBe(3);
  });

  it("recusa livro com id repetido", () => {
    expect(() => bib.cadastrarLivro(new Livro(1, "Clone", "Alguém"))).toThrow(
      IdDuplicadoError,
    );
  });

  it("começa com todos os livros disponíveis", () => {
    expect(bib.disponiveis().map((l) => l.id)).toEqual([1, 2, 3]);
  });

  it("acervo lista todos, emprestados ou não", () => {
    bib.emprestar(1, 10);
    expect(bib.acervo().map((l) => l.id)).toEqual([1, 2, 3]);
  });

  it("emprestar tira o livro dos disponíveis e põe na lista do leitor", () => {
    bib.emprestar(1, 10);
    expect(bib.disponiveis().map((l) => l.id)).toEqual([2, 3]);
    expect(bib.emprestadosPor(10).map((l) => l.id)).toEqual([1]);
    expect(bib.emprestadosPor(20)).toEqual([]);
  });

  it("emprestar livro que não existe lança NaoEncontradoError", () => {
    expect(() => bib.emprestar(99, 10)).toThrow(NaoEncontradoError);
  });

  it("leitor inexistente: NaoEncontradoError e livro intacto", () => {
    expect(() => bib.emprestar(1, 999)).toThrow(NaoEncontradoError);
    expect(bib.disponiveis().map((l) => l.id)).toEqual([1, 2, 3]);
  });

  it("emprestar livro já emprestado lança LivroIndisponivelError", () => {
    bib.emprestar(1, 10);
    expect(() => bib.emprestar(1, 20)).toThrow(LivroIndisponivelError);
  });

  it("terceiro empréstimo com limite 2 é recusado e nada muda", () => {
    bib.emprestar(1, 10);
    bib.emprestar(2, 10);

    expect(() => bib.emprestar(3, 10)).toThrow(LimiteDeEmprestimosError);
    expect(bib.emprestadosPor(10).map((l) => l.id)).toEqual([1, 2]);
    expect(bib.disponiveis().map((l) => l.id)).toEqual([3]);
  });

  it("devolver libera vaga no limite", () => {
    bib.emprestar(1, 10);
    bib.emprestar(2, 10);
    bib.devolver(1);
    bib.emprestar(3, 10);

    expect(bib.emprestadosPor(10).map((l) => l.id)).toEqual([2, 3]);
    expect(bib.disponiveis().map((l) => l.id)).toEqual([1]);
  });

  it.each<[string, number[]]>([
    ["machado", [1, 2]],
    ["tolkien", []],
    [" VIDAS ", [3]],
    ["", []],
    ["   ", []],
  ])("pesquisar(%j) -> %j", (termo, ids) => {
    expect(bib.pesquisar(termo).map((l) => l.id)).toEqual(ids);
  });
});
