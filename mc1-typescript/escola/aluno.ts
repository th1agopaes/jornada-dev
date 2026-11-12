// tipo Situacao
export type Situacao = "aprovado" | "recuperacao" | "reprovado" | "cursando";

// classe Aluno
export class Aluno {
  readonly nome: string;
  #notas: number[] = [];

  constructor(nome: string) {
    this.nome = nome;
  }

  adicionarNota(nota: number): boolean {
    if (Number.isNaN(nota) || nota < 0 || nota > 10) {
      return false;
    }
    this.#notas.push(nota);
    return true;
  }

  get media(): number {
    const soma = this.#notas.reduce((acc, nota) => acc + nota, 0);
    return soma / this.#notas.length;
  }

  get situacao(): Situacao {
    const m = this.media;

    if (Number.isNaN(m)) {
      return "cursando";
    }
    if (m >= 7) {
      return "aprovado";
    }
    if (m >= 5) {
      return "recuperacao";
    }
    return "reprovado";
  }
}
