build:
	wasm-pack build --target web --out-name wasm --out-dir ./static

serve:
	miniserve ./static --index index.html

run: build serve