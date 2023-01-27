
release:
	cd pinyin_svc/; RUSTFLAGS="-C target-cpu=native" cargo build --release --no-default-features

run-release:
	cd pinyin_svc/; RUSTFLAGS="-C target-cpu=native" cargo run --release --no-default-features

build:
	cd pinyin_svc/; cargo build

run:
	cd pinyin_svc/; cargo run

arm-release:
	cd pinyin_svc/; RUSTFLAGS="-C target-cpu=neoverse-n1" cargo build --target=aarch64-unknown-linux-gnu --release --no-default-features
