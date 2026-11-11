const notas: number[] = [8, 7.5, 9];
notas.push(10);
console.log(notas, notas.length);

const dobradas = notas.map((n) => n * 2);
const altas = notas.filter((n) => n >= 8);
const soma = notas.reduce((acumulado, n) => acumulado + n, 0);
console.log(dobradas, altas, soma);

for (const n of notas) {
  console.log(n);
}

const par: [string, number] = ["Cálculo", 8.5];
const [disciplina, nota] = par;
console.log(disciplina, nota);
