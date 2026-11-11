function aplicarBonus(nota: number, bonus = 0.5, limite?: number): number {
  const total = nota + bonus;
  if (limite !== undefined && total > limite) {
    return limite;
  }

  return total;
}

const aplicar = (nota: number, bonus = 0.5, limite?: number): number => {
  const total = nota + bonus;
  return limite !== undefined && total > limite ? limite : total;
};

console.log(aplicarBonus(7));
console.log(aplicarBonus(9.8, 0.5));
console.log(aplicarBonus(9.8, 0.5, 10));
console.log(aplicarBonus(9.8, undefined, 10));

console.log(aplicar(7));
console.log(aplicar(9.8, 0.5));
console.log(aplicar(9.8, 0.5, 10));
console.log(aplicar(9.8, undefined, 10));
