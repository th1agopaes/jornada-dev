interface Descritivel {
  descrever(): string;
}

class Conta {
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

class Poupanca extends Conta implements Descritivel {
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

function mostrar(d: Descritivel): void {
  console.log(d.descrever());
}

const p = new Poupanca("Haru", 0.005);
p.depositar(1000);
p.render();
mostrar(p);
mostrar({ descrever: () => "Sou só um objeto literal" });
console.log(p instanceof Conta, p instanceof Poupanca);
