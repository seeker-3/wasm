const importObject = { console: { log: (x: number) => console.log(x) } }
const bytes = await Deno.readFile('./main.wasm')
await WebAssembly.instantiate(bytes, importObject)
