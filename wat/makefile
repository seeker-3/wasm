run: main wat main.ts
	deno run --allow-read main.ts

wat: main
	wasm2wat main.wasm -o out.wat

memory:
	wasm2wat memory.wasm -o memory.wat

main: main.wat
	wat2wasm main.wat -o main.wasm
