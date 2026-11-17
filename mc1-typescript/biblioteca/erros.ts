export class LivroIndisponivelError extends Error {
  readonly livroId: number;

  constructor(livroId: number) {
    super(`O livro ${livroId} não está disponível`);
    this.name = "LivroIndisponivelError";
    this.livroId = livroId;
  }
}

export class DevolucaoInvalidaError extends Error {
  readonly livroId: number;

  constructor(livroId: number) {
    super(`O livro ${livroId} não está emprestado`);
    this.name = "DevolucaoInvalidaError";
    this.livroId = livroId;
  }
}

export class LimiteDeEmprestimosError extends Error {
  readonly leitorId: number;
  readonly limite: number;

  constructor(leitorId: number, limite: number) {
    super(`O leitor ${leitorId} já tem ${limite} livros emprestados`);
    this.name = "LimiteDeEmprestimosError";
    this.leitorId = leitorId;
    this.limite = limite;
  }
}
