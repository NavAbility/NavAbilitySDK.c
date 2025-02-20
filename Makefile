
NVA_API_URL ?= "https://api.navability.io/graphql"
NVA_API_TOKEN ?= ""

clean:
	cargo clean
	rm -rf test/build
	rm -f src/schema.json

test-tokio:
	cargo test -F tokio

build-tokio:
	cargo build -F tokio

build-lib: build-tokio generate-cbindgen-c generate-cbindgen-cpp

# partial-duplicate of cargo.toml::build-dependencies
install-deps:
	cargo install graphql_client_cli

fetch-schema:
	@graphql-client introspect-schema --authorization $(NVA_API_TOKEN) --output src/schema.json $(NVA_API_URL)

test-capi: build-lib
	cd test && $(MAKE)

generate-cbindgen-cpp:
	cbindgen  --config cbindgen.toml --crate navabilitysdk --output include/NavAbilitySDK.hpp
	cat src/capi/SDKSupplemental.h >> include/NavAbilitySDK.hpp

generate-cbindgen-c:
	cbindgen  --config cbindgen.toml --lang c --crate navabilitysdk --output include/NavAbilitySDK.h
	cat src/capi/SDKSupplemental.h >> include/NavAbilitySDK.h

