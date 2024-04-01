set dotenv-load

make-docs:
	cargo doc --no-deps

web:
	wasm-pack build --target web

trans:
	just web
	cp -r pkg/ $DEST
