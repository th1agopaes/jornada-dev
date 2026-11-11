interface Aluno {
  readonly matricula: string;
  nome: string;
  semestre: number;
  email?: string;
}

const haru: Aluno = {
  matricula: "2024001",
  nome: "Haru",
  semestre: 5,
};

const visitante = {
  matricula: "0",
  nome: "Visitante",
  semestre: 1,
  cidade: "Cuiabá",
};

function apresentar(aluno: Aluno): string {
  return `${aluno.nome} (${aluno.matricula}), ${aluno.semestre}º semestre`;
}

console.log(apresentar(haru));
haru.semestre = 6;
console.log(haru.semestre);
console.log(apresentar(visitante));
