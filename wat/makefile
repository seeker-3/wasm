run:
	deno run --allow-read main.ts

loop: src/loop.wat
	wat2wasm src/loop.wat -o build/loop.wasm

memory: src/memory.wat
	wat2wasm src/memory.wat -o build/memory.wasm

main: src/main.wat
	wat2wasm src/main.wat -o build/main.wasm

clean:
	rm -f build/*
