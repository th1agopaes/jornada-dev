import { Aluno } from "./escola/aluno.ts";
import { Turma } from "./escola/turma.ts";

// Aluno para aprovar >=7
const ana = new Aluno("Ana");
ana.adicionarNota(10);
ana.adicionarNota(9);
console.log("Nota 11 aceita?", ana.adicionarNota(11));

// Aluno para recuperação >=5 e <7
const carlos = new Aluno("Carlos");
carlos.adicionarNota(6);
carlos.adicionarNota(5);

// Aluno sem nota nenhuma
const joao = new Aluno("João");

// matricular os três
const turma = new Turma();
turma.matricular(ana);
turma.matricular(carlos);
turma.matricular(joao);

const alunos = [ana, carlos, joao];
for (const a of alunos) {
  if (a.situacao === "cursando") {
    console.log(`${a.nome} está cursando.`);
  }

  if (a.situacao !== "cursando") {
    console.log(
      `${a.nome} está ${a.situacao} com média ${a.media.toFixed(2)}.`,
    );
  }
}

// nomes dos alunos aprovados turmas.aprovados() devolve Aluno[];
const apr = turma.aprovados();
console.log(`Alunos aprovados: ${apr.map((a) => a.nome).join(", ")}.`);

// média geral da turma
const mg = turma.mediaGeral;
console.log(`Média geral da turma: ${mg.toFixed(2)}.`);
