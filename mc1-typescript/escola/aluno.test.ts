import { describe, expect, it } from "vitest";
import { Aluno, type Situacao } from "./aluno.ts";

describe("Aluno", () => {
  it("aceita nota válida", () => {
    const ana = new Aluno("Ana");
    expect(ana.adicionarNota(10)).toBe(true);
  });

  it("reucsa nota acima de 10, abaixo de 0 e NaN", () => {
    const ana = new Aluno("Ana");
    expect(ana.adicionarNota(11)).toBe(false);
    expect(ana.adicionarNota(-1)).toBe(false);
    expect(ana.adicionarNota(Number.NaN)).toBe(false);
  });

  it("nota recusada não entra na média", () => {
    const ana = new Aluno("Ana");
    ana.adicionarNota(8);
    ana.adicionarNota(11);
    expect(ana.media).toBe(8);
  });

  it("aluno sem notas está cursando", () => {
    const ana = new Aluno("Ana");
    expect(ana.situacao).toBe("cursando");
  });

  it("média com decimais", () => {
    const bia = new Aluno("Bia");
    bia.adicionarNota(0.1);
    bia.adicionarNota(0.2);
    expect(bia.media).toBeCloseTo(0.15);
  });

  it.each<[number[], Situacao]>([
    [[7], "aprovado"],
    [[6.9], "recuperacao"],
    [[5], "recuperacao"],
    [[4.9], "reprovado"],
    [[10, 9], "aprovado"],
  ])("notas %j -> %s", (notas, esperado) => {
    const aluno = new Aluno("Teste");
    for (const nota of notas) {
      aluno.adicionarNota(nota);
    }
    expect(aluno.situacao).toBe(esperado);
  });
});
