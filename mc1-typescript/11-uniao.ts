function formatarNota(valor: number | string): string {
  if (typeof valor === "string") {
    return valor.toUpperCase();
  }
  return valor.toFixed(1);
}

console.log(formatarNota(8.456));
console.log(formatarNota("dispensado"));

type Situacao = "aprovado" | "recuperacao" | "reprovado";

let situacao: Situacao = "aprovado";
console.log(situacao);

type Avaliacao =
  | { tipo: "prova"; nota: number }
  | { tipo: "trabalho"; nota: number; integrantes: string[] };

function descrever(a: Avaliacao): string {
  switch (a.tipo) {
    case "prova":
      return `Prova: ${a.nota}`;
    case "trabalho":
      return `Trabalho de ${a.integrantes.length} pessoas: ${a.nota}`;
  }
}

console.log(descrever({ tipo: "prova", nota: 7 }));
console.log(
  descrever({ tipo: "trabalho", nota: 9, integrantes: ["Haru", "Chocolate"] }),
);
