// biome-ignore lint/suspicious/noExplicitAny: experimento do módulo 2 (any desliga o revisor)
const dadoAny: any = JSON.parse('"haru"');
const dadoUnknown: unknown = JSON.parse('"haru"');

console.log(dadoAny.toUpperCase());
if (typeof dadoUnknown === "string") {
  console.log(dadoUnknown.toUpperCase());
}
