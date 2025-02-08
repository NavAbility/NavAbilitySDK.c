
.DEFAULT_GOAL := run-testc

build-testc: test.c
	mkdir -p build
	gcc -Wall -g -O0 test.c -o build/a.out -I../include/ -L../target/debug/ -lnavabilitysdk
.PHONY: build-testc

run-testc: build-testc
	@LD_LIBRARY_PATH=../target/debug ./build/a.out
.PHONY: run-testc
