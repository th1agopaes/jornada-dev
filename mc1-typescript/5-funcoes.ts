function somar(a: number, b: number): number {
  return a + b;
}
// biome-ignore lint/complexity/useArrowFunction: exemplo das três formas de função (módulo 3)
const subtrair = function (a: number, b: number): number {
  return a - b;
};
const multiplicar = (a: number, b: number): number => a * b;

console.log(somar(2, 3), subtrair(5, 2), multiplicar(4, 3));
