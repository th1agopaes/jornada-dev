// requisito 1 (type Situacao)
type Situacao = "aprovado" | "recuperacao" | "reprovado" | "cursando";

// requisito 2 (interface Disciplina)
interface Disciplina {
  readonly codigo: string;
  nome: string;
  notas: number[];
  professor?: string;
}

// requisito 3 (function media)
function media(notas: number[]): number {
  const soma = notas.reduce((acumulado, n) => acumulado + n, 0);
  return soma / notas.length;
}

// requisito 4 (situacao)
function situacao(m: number): Situacao {
  if (Number.isNaN(m)) {
    return "cursando";
  }
  if (m >= 6) {
    return "aprovado";
  }
  if (m >= 4) {
    return "recuperacao";
  }

  return "reprovado";
}

// requisito 5 (resumo)
function resumo(d: Disciplina): [string, number, Situacao] {
  const m = media(d.notas);
  return [d.nome, m, situacao(m)];
}

// requisito 6 (dados, laço e contagem)
const disciplinas: Disciplina[] = [
  {
    codigo: "CAL3",
    nome: "Cálculo III",
    notas: [8, 7.5, 9],
    professor: "Marta",
  },

  { codigo: "FIS2", nome: "Física II", notas: [5, 4.5, 6] },
  { codigo: "RED1", nome: "Redes I", notas: [2, 3.5, 4] },
  { codigo: "EST1", nome: "Estágio", notas: [] },
];

for (const d of disciplinas) {
  const [nome, m, sit] = resumo(d);
  console.log(
    `Disciplina: ${nome}, Média: ${sit === "cursando" ? "N/A" : m.toFixed(1)}, Situação: ${sit}`,
  );
}

const aprovadas = disciplinas.filter(
  (d) => situacao(media(d.notas)) === "aprovado",
);
console.log(`Total de disciplinas aprovadas: ${aprovadas.length}`);
