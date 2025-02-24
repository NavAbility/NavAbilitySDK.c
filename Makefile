
SHELL := /bin/bash

bold := $(shell tput bold)
sgr0 := $(shell tput sgr0)

NVA_API_TOKEN ?= ""
NVA_API_URL ?= "https://api.navability.io/graphql"
NVA_PWA_URL ?= "https://app.navability.io"
WHICHBROWSER=$(shell xdg-settings get default-web-browser)

default: help ;
.PHONY: default

clean:
	cargo clean
	rm -rf test/build
	rm -f src/schema.json
.PHONY: clean

build-tokio:
	cargo build -F tokio
.PHONY: build-tokio

build-lib: build-tokio generate-cbindgen-c generate-cbindgen-cpp

fetch-schema:
	@graphql-client introspect-schema --authorization $(NVA_API_TOKEN) --output src/schema.json $(NVA_API_URL)
.PHONY: fetch-schema

install-sys-deps:
	sudo apt install curl pkg-config libssl-dev xclip -y

install-rust: install-sys-deps
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# partial-duplicate of cargo.toml::build-dependencies
install-rust-deps:
	@echo "Remember to first run 'make install-rust' or 'make install-sys-deps' if you have not already done so, or check this Makefile if you unsure of the impact."
	cargo install graphql_client_cli

generate-cbindgen-cpp:
	cbindgen  --config cbindgen.toml --crate navabilitysdk --output include/NavAbilitySDK.hpp
	cat src/capi/SDKSupplemental.h >> include/NavAbilitySDK.hpp

generate-cbindgen-c:
	cbindgen  --config cbindgen.toml --lang c --crate navabilitysdk --output include/NavAbilitySDK.h
	cat src/capi/SDKSupplemental.h >> include/NavAbilitySDK.h

test-capi: build-lib
	cd test && $(MAKE)

update-api-token: default-browser-firefox-api default-browser-chromium-api default-browser-gchrome-api
	@echo ""
	@echo "Find the newly opened browser and log in to Nva App, then run the following command in your terminal:"
	@echo "$(bold)export NAVABILITY_API_TOKEN=<paste your token here>$(sgr0)"

# Copy output of command to clipboard
nva-api-url-to-clipboard:
	@echo "Dumping the contents of env variable NVA_API_URL into the clipboard"
	@echo $(NVA_API_URL) | tr -d '\n' | xclip -selection clipboard

# Copy output of command to clipboard
nva-api-token-to-clipboard:
	@echo "Dumping the contents of env variable NVA_API_TOKEN into the clipboard"
	@echo $(NVA_API_TOKEN) | tr -d '\n' | xclip -selection clipboard
	
# Retrieve text from clipboard
nva-api-token-from-clipboard:
	@echo "Dumping the clipboard into env variable NVA_API_TOKEN"
	$(NVA_API_TOKEN)=$(xclip -o -selection clipboard)

# Retrieve text from clipboard
nva-org-id-from-clipboard:
	@echo "Dumping the clipboard into env variable NVA_API_TOKEN"
	$(NVA_ORG_ID)=$(xclip -o -selection clipboard)

fetch-org-id-to-clipboard:
	@echo "Fetching org-id and dumping into the clipboard: $(NVA_API_URL)"
	@curl -s $(NVA_API_URL) \
		-X POST \
		-H 'Authorization: Bearer $(NVA_API_TOKEN)' \
		-H 'content-type: application/json' \
		--data '{ "query": "{ orgs { id } }" }' | grep -o -P "[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}" | tr -d '\n' | xclip -selection clipboard

info: help
.PHONY: info

help:
	@echo ""
	@echo "                                        .......             "
	@echo "                                      ............          "
	@echo "                                    ........... ...         "
	@echo "                                   ... .       . ...        "
	@echo "                                 ......         . .. .      "
	@echo "                              ............      . .. ....   "
	@echo "                             ... ...... ....   . ... . .... "
	@echo "                            .....      ....... ....   ..... "
	@echo "                           ... .         .........     . ..."
	@echo "                           ... .      .........        . ..."
	@echo "                            .....   .... .......      ..... "
	@echo "                             ... ..... .   .... ...... .... "
	@echo "                              .... ....      ............   "
	@echo "                                 . ....         ......      "
	@echo "                                   ... .       . ...        "
	@echo "                                    .... ...... ...         "
	@echo "                                      ............          "
	@echo "                                        .......             "
	@echo ""
	@echo "Welcome to the command line interface (CLI) for NavAbilitySDK."
	@echo ""
	@echo "The NavAbilitySDKs provide machine/human friendly access to NavAbility(TM) Accelerator,"
	@echo "  including various use cases: real-time, in-situ, interactive, batch, and analysis."
	@echo "  Similar SDKs are available for popular programming languages:"
	@echo "  - https://github.com/NavAbility"
	@echo ""
	@echo "COMPILED BINARIES:"
	@echo " Downloads (.so/.h)"
	@echo "  - C-Compliant .deb (Debian/Ubuntu): https://www.wherewhen.ai/_tba_ (Coming Feb2025!)"
	@echo "    - examples: https://github.com/NavAbility/NavAbilitySDK.c"
	@echo " General Docs: "
	@echo "  - https://navability.github.io/NavAbilitySDK.py/"
	@echo ""
	@echo ""
	@echo "------------ Hello: ------------"
	@echo "To use, run:  make [task]:"
	@echo ""
	@echo "- make [help|info]         // this documentation, also the default printout"
	@echo ""
	@echo "Get a token from the human-friendly App"
	@echo ""
	@echo "- make update-api-token    // update env var NAV_API_TOKEN (browser login)"
	@echo ""
	@echo "    $$""NVA_PWA_URL = "$(NVA_PWA_URL)
	@echo "    $$""NVA_API_TOKEN <- copy-paste-24hrs" 
	@echo ""
	@echo "--------- Dependencies: ---------"
	@echo "The following commands are necessary for building the source code (Linux):"
	@echo ""
	@echo "- make install-sys-deps    // sudo apt install -y"
	@echo "- make install-rust        // rustup www.rust-lang.org"
	@echo "- make install-rust-deps   // cargo install"
	@echo ""
	@echo "----------- Compile: -----------"
	@echo "For targets native (x86, ARM, ..):"
	@echo "  Rust type-safety implies/requires env vars: NVA_API_URL/TOKEN"
	@echo "   (also see compiled binaries above)"
	@echo ""
	@echo "- make build-lib"
	@echo ""
	@echo ""
	@echo "NavAbility(TM) Accelerator by WhereWhen.ai Technologies Inc."  
	@echo "--------------------------"
	@echo "Our mission is to empower teams to bring robotic-like systems to the market faster, and accelerate"
	@echo "  the data transformation of large asset industries with bespoke spatial intelligence software."
	@echo "WhereWhen.ai develops and supports open-source science via permissive licenses of our core algorithms"
	@echo "  to simplify, amplify, and encourage more access to mission critical navigation-like (GNC) software."
	@echo ""
	@echo "See more about our process here:"
	@echo "  - https://www.wherewhen.ai/post/the-navability-logo"
	@echo "  - https://github.com/JuliaRobotics/Caesar.jl" 
	@echo ""
	@echo "For more information or help please contact us at, or open an issue at:"
	@echo "  - https://github.com/NavAbility/NavAbilitySDK.c/issues/new"
	@echo "  - info@wherewhen.ai"
	@echo ""
.PHONY: help
