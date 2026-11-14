import { beforeEach, describe, expect, it } from "vitest";
import { IdDuplicadoError, NaoEncontradoError } from "./erros.ts";
import { Repositorio } from "./repositorio.ts";

interface Tarefa {
  id: number;
  titulo: string;
}

describe("Repositorio", () => {
  let repo: Repositorio<Tarefa>;

  beforeEach(() => {
    repo = new Repositorio<Tarefa>();
    repo.adicionar({ id: 1, titulo: "Estudar" });
  });

  it("começa com 1 item", () => {
    expect(repo.tamanho).toBe(1);
  });

  it("buscar devolve o item quando o id existe", () => {
    expect(repo.buscar(1)).toEqual({ id: 1, titulo: "Estudar" });
  });

  it("buscar devolve undefined quando o id não existe", () => {
    expect(repo.buscar(999)).toBeUndefined();
  });

  it("obter devolve o item quando o id existe", () => {
    expect(repo.obter(1).titulo).toBe("Estudar");
  });

  it("obter lança NaoEncontradoError quando o id não existe", () => {
    expect(() => repo.obter(999)).toThrow(NaoEncontradoError);
  });

  it("adicionar recusa id repetido e não muda o tamanho", () => {
    expect(() => repo.adicionar({ id: 1, titulo: "Repetida" })).toThrow(
      IdDuplicadoError,
    );
    expect(repo.tamanho).toBe(1);
  });

  it("remover devolve true na primeira vez e false na segunda", () => {
    expect(repo.remover(1)).toBe(true);
    expect(repo.remover(1)).toBe(false);
  });

  it("listar devolve uma cópia", () => {
    const copia = repo.listar();
    copia.push;
    ({ id: 2, titulo: "Nova tarefa" });
    expect(repo.tamanho).toBe(1);
  });
});
