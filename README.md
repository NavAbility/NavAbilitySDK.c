# NavAbilitySDK.c

Copyright 2025, NavAbility Contributors.  This repo is licensed according to Apache 2.0.  See the LICENSE file.

[![CI](https://github.com/NavAbility/NavAbilitySDK.c/actions/workflows/ci-c.yml/badge.svg)](https://github.com/NavAbility/NavAbilitySDK.c/actions/workflows/ci-c.yml)

> [!IMPORTANT]
> This repo derives from upstream branches at NavAbility/NavAbilitySDK.rs e.g. `develop->develop`.  Changes for C export are local to SDK.c only.  See git stategy here https://stackoverflow.com/a/37104851, and accidental upstream protection https://stackoverflow.com/a/7556269 (note change `--push upstream`).  Also, simplify the git pull process via the `config branch._` commands here https://stackoverflow.com/a/60724734.

## Introduction

Access NavAbility Accelerator features from C/C++.  See related multi-language SDKs at Github.com/NavAbility/NavAbilitySDK.*.

## Docs

Documentation for [Python](https://navability.github.io/NavAbilitySDK.py/) or [Julia](https://navability.github.io/NavAbilitySDK.jl/dev/) versions exist, work in progress to port Docs for Rust crates (25Q1).

## Compiling

Get deps
```shell
make install-deps # modifies system cargo crates
```

## Exporting Shared Library

Build the shared library:
Set required NVA_API_URL and NVA_API_TOKEN args/env variables and compile for either native or wasm:
```shell
make build-lib
```

Or run the `test/test.c` file with
```shell
make test-capi
```
