console.log("[conta.ts carregado]");

export interface Descritivel {
  descrever(): string;
}
export const TAXA_PADRAO = 0.005;
export class Conta {
  readonly titular: string;
  private limite = 100; // privado só para o tsc
  #saldo = 0; // privado de verdade (JavaScript)

  constructor(titular: string) {
    this.titular = titular;
  }

  depositar(valor: number): void {
    if (valor <= 0) return;
    this.#saldo += valor;
  }

  sacar(valor: number): boolean {
    if (valor > this.#saldo + this.limite) return false;
    this.#saldo -= valor;
    return true;
  }

  get saldo(): number {
    return this.#saldo;
  }
}
export class Poupanca extends Conta implements Descritivel {
  taxa: number;

  constructor(titular: string, taxa: number) {
    super(titular);
    this.taxa = taxa;
  }

  render(): void {
    this.depositar(this.saldo * this.taxa);
  }

  descrever(): string {
    return `Poupança de ${this.titular}: R$ ${this.saldo.toFixed(2)}`;
  }
}
