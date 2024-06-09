.PHONY: build

build:
	rustc -C target-feature=+crt-static src/main.rs -o rsd

docker:
	docker buildx build -t rsd:latest .

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

test:
	cargo test --all --release

release:
	cargo build --all --release

sbom:
	syft -o spdx-json . | jq . > sbom.json
