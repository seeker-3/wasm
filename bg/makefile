
build: src/lib.rs
	wasm-pack build --dev --target web --out-dir public/pkg  --no-typescript --out-name index

clean:
	rm -rf public/pkg

serve:
	npx live-server public