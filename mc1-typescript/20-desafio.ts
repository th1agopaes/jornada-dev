import { IdDuplicadoError, NaoEncontradoError } from "./repo/erros.ts";
import { Repositorio } from "./repo/repositorio.ts";

interface Tarefa {
  id: number;
  titulo: string;
  feita: boolean;
  prazo?: string;
}

const tarefas = new Repositorio<Tarefa>();

tarefas.adicionar({ id: 1, titulo: "Estudar genéricos", feita: true });
tarefas.adicionar({
  id: 2,
  titulo: "Fazer o desafio",
  feita: false,
  prazo: "12/11",
});
console.log(`Total: ${tarefas.tamanho}`);

console.log(tarefas.buscar(2)?.titulo ?? "não encontrada");
console.log(tarefas.buscar(99)?.titulo ?? "não encontrada");
console.log(`Prazo da 1: ${tarefas.obter(1).prazo ?? "sem prazo"}`);

function executar(descricao: string, acao: () => void): void {
  try {
    acao();
    console.log(`✔ ${descricao}`);
  } catch (erro) {
    if (
      erro instanceof IdDuplicadoError ||
      erro instanceof NaoEncontradoError
    ) {
      console.log(`✖ ${descricao} → ${erro.name}: ${erro.message}`);
    } else {
      throw erro;
    }
  }
}

executar("obter a 2", () => {
  tarefas.obter(2);
});
executar("obter a 99", () => {
  tarefas.obter(99);
});
executar("adicionar id repetido", () => {
  tarefas.adicionar({ id: 1, titulo: "Clone", feita: false });
});

console.log(`Removeu a 1? ${tarefas.remover(1)}`);
console.log(`Removeu a 1 de novo? ${tarefas.remover(1)}`);

const copia = tarefas.listar();
copia.push({ id: 50, titulo: "Intrusa", feita: false });
console.log(`Total depois da intrusa: ${tarefas.tamanho}`);

for (const t of tarefas.listar()) {
  console.log(`#${t.id} ${t.titulo} [${t.feita ? "x" : " "}]`);
}
