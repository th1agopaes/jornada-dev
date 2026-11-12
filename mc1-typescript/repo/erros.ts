export class NaoEncontradoError extends Error {
  readonly id: number;

  constructor(id: number) {
    super(`Nenhum item com id ${id}`);
    this.name = "NaoEncontradoError";
    this.id = id;
  }
}

export class IdDuplicadoError extends Error {
  readonly id: number;

  constructor(id: number) {
    super(`Já existe item com id ${id}`);
    this.name = "IdDuplicadoError";
    this.id = id;
  }
}
