const dadoAny: any = JSON.parse('"haru"');
const dadoUnknown: unknown = JSON.parse('"haru"');

if (typeof dadoUnknown == "string") {
  console.log(dadoUnknown.toUpperCase());
}
