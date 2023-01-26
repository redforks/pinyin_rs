
release:
	RUSTFLAGS="-C target-cpu=native" cargo build --release --no-default-features

run-release:
	RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features

build:
	RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo build

run:
	RUSTFLAGS="-C link-arg=-fuse-ld=mold" cargo run
