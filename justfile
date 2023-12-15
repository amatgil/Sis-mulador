make-docs:
	cargo doc --no-deps


package:
	rm -rf pkg/
	cargo build --release
	just make-docs
	mkdir pkg/
	cp -r ./target/release/sICmulador pkg/
	cp -r ./target/doc/sICmulador/ pkg/
	cp -r ./examples pkg/
