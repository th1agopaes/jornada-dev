import { Aluno } from "./aluno.ts";

export class Turma {
  #alunos: Aluno[] = [];

  matricular(aluno: Aluno): void {
    this.#alunos.push(aluno);
  }

  aprovados(): Aluno[] {
    return this.#alunos.filter((a) => a.situacao === "aprovado");
  }

  get mediaGeral(): number {
    const comNotas = this.#alunos.filter((a) => a.situacao !== "cursando");
    const soma = comNotas.reduce((acc, a) => acc + a.media, 0);
    return soma / comNotas.length;
  }
}
