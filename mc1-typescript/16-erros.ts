class NotaInvalidaError extends Error {
  readonly nota: number;

  constructor(nota: number) {
    super(`Nota inválida: ${nota} (use de 0 a 10)`);
    this.name = "NotaInvalidaError";
    this.nota = nota;
  }
}

function validarNota(nota: number): number {
  if (Number.isNaN(nota) || nota < 0 || nota > 10) {
    throw new NotaInvalidaError(nota);
  }
  return nota;
}

function tentar(nota: number): void {
  try {
    const valida = validarNota(nota);
    console.log(`ok: ${valida}`);
  } catch (erro) {
    if (erro instanceof NotaInvalidaError) {
      console.log(`nota recusada (${erro.nota}): ${erro.message}`);
    } else if (erro instanceof Error) {
      console.log(`erro inesperado: ${erro.message}`);
    } else {
      console.log("jogram algo que nem é Error:", erro);
    }
  } finally {
    console.log("-- fim da tentativa --");
  }
}

tentar(8);
tentar(11);
tentar(NaN);
