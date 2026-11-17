import { DevolucaoInvalidaError, LivroIndisponivelError } from "./erros.ts";

// União discriminada (módulo 4): a etiqueta é o campo tipo.
export type Situacao =
  | { tipo: "disponivel" }
  | { tipo: "emprestado"; leitorId: number };

export function descreverSituacao(situacao: Situacao): string {
  switch (situacao.tipo) {
    case "disponivel":
      return "disponível";
    case "emprestado":
      return `emprestado ao leitor ${situacao.leitorId}`;
  }
}

export class Livro {
  readonly id: number;
  readonly titulo: string;
  readonly autor: string;
  #situacao: Situacao = { tipo: "disponivel" };

  constructor(id: number, titulo: string, autor: string) {
    this.id = id;
    this.titulo = titulo;
    this.autor = autor;
  }

  get situacao(): Situacao {
    return this.#situacao;
  }

  get disponivel(): boolean {
    return this.#situacao.tipo === "disponivel";
  }

  emprestar(leitorId: number): void {
    if (this.#situacao.tipo !== "disponivel") {
      throw new LivroIndisponivelError(this.id);
    }
    this.#situacao = { tipo: "emprestado", leitorId };
  }

  devolver(): number {
    if (this.#situacao.tipo === "disponivel") {
      throw new DevolucaoInvalidaError(this.id);
    }
    const leitorId = this.#situacao.leitorId;
    this.#situacao = { tipo: "disponivel" };
    return leitorId;
  }
}
