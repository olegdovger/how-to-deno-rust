import loader from "./rust-wasm-loader.ts";

const math = await loader("math");

console.log(math.factorial(10));
