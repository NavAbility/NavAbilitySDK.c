# NavAbilitySDK.c

Copyright 2025, NavAbility Contributors.  This repo is licensed according to Apache 2.0.  See the LICENSE file.

[![CI](https://github.com/NavAbility/NavAbilitySDK.c/actions/workflows/ci-c.yml/badge.svg)](https://github.com/NavAbility/NavAbilitySDK.c/actions/workflows/ci-c.yml)


## Introduction

Access NavAbility Accelerator features from C/C++.  See related multi-language SDKs at Github.com/NavAbility/NavAbilitySDK.*.

## Docs

Documentation for [Python](https://navability.github.io/NavAbilitySDK.py/) or [Julia](https://navability.github.io/NavAbilitySDK.jl/dev/) versions exist, work in progress to port Docs for Rust crates (25Q1).

## First time setup (cloning)

This repo derives from upstream branches at NavAbility/NavAbilitySDK.rs:develop, which by convention is locally used in git as `upstream`.  Regular `origin` for this repo holds the C-extras that take SDK.rs to have everything necessary for SDK.c.  When cloning the repo, also do this:
```
git clone <this https/ssh>
cd NavAbilitySDK.c
git remote add upstream <https or ssh github.com/NavAbilitySDK.rs>
git remote set-url --push upstream no-pushing
```

The last instruction protects the upstream SDK.rs from receiving C related code directly.  Any work required on SDK.rs should be done there as normal.

> Useful references on git stategy at https://stackoverflow.com/a/37104851, and stopping accidental upstream protection https://stackoverflow.com/a/7556269 (note change `--push upstream`).  Also, simplify the git pull process via the `config branch._` commands here https://stackoverflow.com/a/60724734.

### Regular git pull

You should push and pull against SDK.c repo as normal
```
git pull
git pull origin # the default
```

But fold in upstream changes into local PR via,
```
git checkout -b 25Q1/ff/somefix
git pull upstream develop
git push origin 25Q1/ff/somefix
```

Note the default branch is `develop`, but could also be stable such as pulling from `upstreams` branch `release/v0.X`.


### System Dependencies

Get deps
```shell
make install-sys-deps # modifies system cargo crates
```

## Compiling Shared Library

Build the shared library:
Set required NVA_API_URL and NVA_API_TOKEN args/env variables and compile for either native or wasm:
```shell
make build-lib
```

### Examples

See `examples/Makefile`.


Or run the `test/test.c` file with
```shell
make test-capi
```
