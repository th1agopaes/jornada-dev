function saudar(nome: string, apelido?: string): string {
  if (apelido !== undefined) {
    return `Olá, ${nome} (${apelido})!`;
  }
  return `Olá, ${nome}!`;
}

function situacao(nota: number, minima = 6): string {
  return nota >= minima ? "aprovado" : "reprovado";
}

function mediaDe(...notas: number[]): number {
  let soma = 0;
  for (const nota of notas) {
    soma += nota;
  }
  return soma / notas.length;
}

console.log(saudar("Thiago"), saudar("Thiago", "Haru"));
console.log(situacao(5.5), situacao(5.5, 5));
console.log(mediaDe(7, 8.5, 9));
console.log(mediaDe());
