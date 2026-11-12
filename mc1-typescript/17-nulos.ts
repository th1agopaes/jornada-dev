interface Aluno {
  nome: string;
  email?: string;
  orientador: string | null;
}

const alunos: Aluno[] = [
  { nome: "Haru", email: "haru@exemplo.com", orientador: null },
  { nome: "Ana", orientador: "Profa. Lia" },
];

function buscar(nome: string): Aluno | undefined {
  return alunos.find((a) => a.nome === nome);
}

const ana = buscar("Ana");
const fantasma = buscar("Zé");

console.log(ana?.email);
console.log(ana?.email?.toUpperCase());
console.log(fantasma?.nome);
console.log(fantasma?.nome ?? "não encontrado");
console.log(ana?.orientador ?? "sem orientador");

const nota = 0;
console.log(nota || "sem nota");
console.log(nota ?? "sem nota");

if (ana !== undefined) {
  console.log(ana.nome.toUpperCase());
}
