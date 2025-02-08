
NVA_API_URL ?= "https://api.navability.io/graphql"
NVA_API_TOKEN ?= ""

clean:
	cargo clean
	rm -rf test/build

install-deps:
	cargo install graphql_client_cli --force

build-wasm:
	cargo build -F wasm

build-tokio:
	cargo build -F tokio

fetch-schema:
	@graphql-client introspect-schema --authorization $(NVA_API_TOKEN) --output src/schema.json $(NVA_API_URL)

test-tokio:
	cargo test -F tokio
