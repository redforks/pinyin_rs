
release:
	RUSTFLAGS="-C target-cpu=native" cargo build --release --no-default-features 

run-release:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features 
build:
	cargo build 

run:
	cargo run 
