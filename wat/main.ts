import { fileName, importObject } from './lib/loop.ts'

const bytes = await Deno.readFile(fileName)
await WebAssembly.instantiate(bytes, importObject)
