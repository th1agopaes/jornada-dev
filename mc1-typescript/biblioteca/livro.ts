import { DevolucaoInvalidaError, LivroIndisponivelError } from "./erros.ts";

// União discriminada (módulo 4): a etiqueta é o campo tipo.
export type Situacao =
  | { tipo: "disponivel" }
  | { tipo: "emprestado"; leitorId: number };
