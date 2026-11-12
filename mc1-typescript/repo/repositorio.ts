import { IdDuplicadoError, NaoEncontradoError } from "./erros.ts";

export interface ComId {
  id: number;
}

export class Repositorio<T extends ComId> {
  #itens: T[] = [];

  get tamanho(): number {
    return this.#itens.length;
  }

  buscar(id: number): T | undefined {
    return this.#itens.find((i) => i.id === id);
  }

  adicionar(item: T): void {
    if (this.#itens.some((i) => i.id === item.id)) {
      throw new IdDuplicadoError(item.id);
    }
    this.#itens.push(item);
  }

  obter(id: number): T {
    const item = this.buscar(id);
    if (item === undefined) {
      throw new NaoEncontradoError(id);
    }
    return item;
  }

  remover(id: number): boolean {
    const indice = this.#itens.findIndex((i) => i.id === id);
    if (indice === -1) {
      return false;
    }
    this.#itens.splice(indice, 1);
    return true;
  }

  listar(): T[] {
    return [...this.#itens];
  }
}
