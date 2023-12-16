make-docs:
	cargo doc --no-deps


pkg:
	rm -rf pkg/
	cargo build --release
	just make-docs
	mkdir -p pkg/
	cp ./target/release/sICmulador pkg/
	cp -r ./examples pkg/
	cp -r ./target/doc/ pkg/ # Add docs, TODO: Make proper
