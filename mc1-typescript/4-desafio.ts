const valor1 = 7;
const valor2 = "8.5";
const valor3 = true;

function paraNumero(valor: unknown): number {
  if (typeof valor === "number") {
    return valor;
  }

  if (typeof valor === "string") {
    return Number(valor);
  }

  return 0;
}

console.log(paraNumero(valor1));
console.log(paraNumero(valor2));
console.log(paraNumero(valor3));
