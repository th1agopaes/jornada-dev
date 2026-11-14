import { describe, expect, it } from "vitest";
import { Aluno } from "./aluno.ts";
import { Turma } from "./turma.ts";

describe("Turma", () => {
  it("mediaGeral é undefined quando ninguém tem nota", () => {
    // Arrange
    const turma = new Turma();
    turma.matricular(new Aluno("João"));

    // Act + Assert
    expect(turma.mediaGeral).toBeUndefined();
  });

  it("mediaGeral ignora quem está cursando", () => {
    const ana = new Aluno("Ana");
    ana.adicionarNota(10);
    ana.adicionarNota(9);
    const carlos = new Aluno("Carlos");
    carlos.adicionarNota(6);
    carlos.adicionarNota(5);
    const turma = new Turma();
    turma.matricular(ana);
    turma.matricular(carlos);
    turma.matricular(new Aluno("Sem nota"));

    expect(turma.mediaGeral).toBe((10 + 9 + 6 + 5) / 4);
  });

  it("aprovados devolve só quem passou", () => {
    // Arrange: Ana com 10 e Carlos com 5, os dois matriculados
    const ana = new Aluno("Ana");
    ana.adicionarNota(10);
    const carlos = new Aluno("Carlos");
    carlos.adicionarNota(5);
    const turma = new Turma();
    turma.matricular(ana);
    turma.matricular(carlos);

    expect(turma.aprovados().map((a) => a.nome)).toEqual(["Ana"]);
  });
});
