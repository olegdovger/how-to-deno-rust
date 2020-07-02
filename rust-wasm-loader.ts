export default async function load(pkgName: String) {
  const wasmCode = await Deno.readFile(
    `./lib/${pkgName}/pkg/${pkgName}_bg.wasm`,
  );
  const wasmModule = new WebAssembly.Module(wasmCode);
  const wasmInstance = new WebAssembly.Instance(wasmModule);

  return wasmInstance.exports;
}
