const nome: string = "Haru";
const semestre: number = 5;

const n1: number = 7.5;
const n2: number = 9;

function saudacao(pessoa: string, periodo: number): string {
  return `Olá, ${pessoa}! Você está no ${periodo}º semestre.`;
}

function media(nota1: number, nota2: number): number {
  return (nota1 + nota2) / 2;
}

console.log(saudacao(nome, semestre));
console.log(`Sua média é: ${media(n1, n2)}`);
