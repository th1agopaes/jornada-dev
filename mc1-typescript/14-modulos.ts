import { type Descritivel, Poupanca, TAXA_PADRAO } from "./banco/conta.ts";

function mostrar(d: Descritivel): void {
  console.log(d.descrever());
}

const p = new Poupanca("Haru", TAXA_PADRAO);
p.depositar(1000);
p.render();
mostrar(p);
