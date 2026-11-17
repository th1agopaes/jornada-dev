import { beforeEach, describe, expect, it } from "vitest";
import { DevolucaoInvalidaError, LivroIndisponivelError } from "./erros.ts";
import { descreverSituacao, Livro, type Situacao } from "./livro.ts";

describe("Livro", () => {
  let livro: Livro;

  beforeEach(() => {
    livro = new Livro(1, "Dom Casmurro", "Machado de Assis");
  });

  it("começa disponível", () => {
    expect(livro.situacao).toEqual({ tipo: "disponivel" });
    expect(livro.disponivel).toBe(true);
  });

  it("emprestar guarda quem pegou", () => {
    livro.emprestar(7);
    expect(livro.situacao).toEqual({ tipo: "emprestado", leitorId: 7 });
    expect(livro.disponivel).toBe(false);
  });

  it("emprestar de novo lança LivroIndisponivelError e mantém o primeiro leitor", () => {
    livro.emprestar(7);
    expect(() => livro.emprestar(8)).toThrow(LivroIndisponivelError);
    expect(livro.situacao).toEqual({ tipo: "emprestado", leitorId: 7 });
  });

  it("devolver devolve o id de quem estava com o livro e o libera", () => {
    livro.emprestar(7);
    expect(livro.devolver()).toBe(7);
    expect(livro.disponivel).toBe(true);
  });

  it("devolver livro disponível lança DevolucaoInvalidaError", () => {
    expect(() => livro.devolver()).toThrow(DevolucaoInvalidaError);
  });

  it("a mensagem do erro diz qual livro", () => {
    livro.emprestar(7);
    expect(() => livro.emprestar(8)).toThrow("O livro 1 não está disponível");
  });
});

describe("descreverSituacao", () => {
  it.each<[Situacao, string]>([
    [{ tipo: "disponivel" }, "disponível"],
    [{ tipo: "emprestado", leitorId: 3 }, "emprestado ao leitor 3"],
  ])("%j -> %s", (situacao, esperado) => {
    expect(descreverSituacao(situacao)).toBe(esperado);
  });
});
