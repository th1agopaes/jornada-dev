function aplicarBonus(nota: number, bonus = 0.5, limite?: number): number {
  if (limite !== undefined && nota + bonus > limite) {
    return limite;
  }

  return nota + bonus;
}

const bonus = (nota: number, bonus = 0.5, limite?: number): number =>
  limite !== undefined && nota + bonus > limite ? limite : nota + bonus;

console.log(aplicarBonus(7));
console.log(aplicarBonus(9.8, 0.5));
console.log(aplicarBonus(9.8, 0.5, 10));

console.log(bonus(7));
console.log(bonus(9.8, 0.5));
console.log(bonus(9.8, 0.5, 10));
