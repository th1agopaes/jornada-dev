function primeiro<T>(itens: T[]): T | undefined {
  return itens[0];
}

const n = primeiro([7, 8, 9]);
const s = primeiro(["a", "b"]);
const vazio = primeiro<string>([]);
console.log(n, s, vazio);

function maisLongo<T extends { length: number }>(a: T, b: T): T {
  return a.length >= b.length ? a : b;
}
console.log(maisLongo("abc", "de"));
console.log(maisLongo([1], [1, 2]));

type Resultado<T> = { ok: true; valor: T } | { ok: false; erro: string };

function paraNota(texto: string): Resultado<number> {
  const numero = Number(texto);
  if (Number.isNaN(numero) || numero < 0 || numero > 10) {
    return { ok: false, erro: `"${texto}" não é nota válida` };
  }
  return { ok: true, valor: numero };
}

for (const texto of ["8.5", "abc", "11"]) {
  const r = paraNota(texto);
  if (r.ok) {
    console.log(`nota ${r.valor.toFixed(1)}`);
  } else {
    console.log(`erro: ${r.erro}`);
  }
}

class Pilha<T> {
  #itens: T[] = [];

  empilhar(item: T): void {
    this.#itens.push(item);
  }

  desempilhar(): T | undefined {
    return this.#itens.pop();
  }

  get tamanho(): number {
    return this.#itens.length;
  }
}

const pilha = new Pilha<string>();
pilha.empilhar("a");
pilha.empilhar("b");
console.log(pilha.desempilhar(), pilha.tamanho);
