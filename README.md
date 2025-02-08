# NavAbilitySDK.c

Copyright 2025, NavAbility Contributors.  This repo is licensed according to Apache 2.0.  See the LICENSE file.

[![CI](https://github.com/NavAbility/NavAbilitySDK.c/actions/workflows/ci.yml/badge.svg)](https://github.com/NavAbility/NavAbilitySDK.c/actions/workflows/ci.yml)

> [!IMPORTANT]
> This repo derives from upstream branches at NavAbility/NavAbilitySDK.rs e.g. `develop->develop`.  Changes for C export are local to SDK.c only.  See git stategy here https://stackoverflow.com/a/37104851.

## Introduction

Access NavAbility Accelerator features from C/C++.  See related multi-language SDKs at Gihub.com/NavAbility/NavAbilitySDK.*.

## Docs

Documentation for [Python](https://navability.github.io/NavAbilitySDK.py/) or [Julia](https://navability.github.io/NavAbilitySDK.jl/dev/) versions exist, work in progress to port Docs for Rust crates (25Q1).

## Compiling

Get the schema with NVA_API_URL and NVA_API_TOKEN args/env var set:
```shell
make install-deps # modifies system cargo crates
make fetch-schema
```

## Exporting Shared Library

Build the shared library:
```shell
make build-lib
```

Or run the `test/test.c` file with
```shell
make test-capi
```