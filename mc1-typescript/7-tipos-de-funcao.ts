type Operacao = (a: number, b: number) => number;

const potencia: Operacao = (base, expoente) => base ** expoente;

function calcular(op: Operacao, a: number, b: number): number {
  return op(a, b);
}

function avisar(mensagem: string): void {
  console.log(`⚠ ${mensagem}`);
}

console.log(calcular(potencia, 2, 10));
console.log(calcular((a, b) => a % b, 17, 5));
avisar("fim dos cálculos");
