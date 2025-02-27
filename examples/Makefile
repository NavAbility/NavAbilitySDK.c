
.DEFAULT_GOAL := run-basic-graph

build-basic-graph: basic_graph_and_solve.c
	mkdir -p build
	gcc -Wall -g -O0 basic_graph_and_solve.c -o build/a.out -I../include/ -L../target/examples/ -lnavabilitysdk
.PHONY: build-basic-graph

run-basic-graph: build-basic-graph
	@LD_LIBRARY_PATH=../target/examples ./build/a.out
.PHONY: run-basic-graph
