
.DEFAULT_GOAL := run-all

#  run-basic-graph
run-all: run-camera-lidar
	@echo "All examples have been run successfully."
.PHONY: run-all

build-all: build-basic-graph build-camera-lidar
	@echo "All examples have been built successfully."
.PHONY: build-all

build-basic-graph: basic_graph_and_solve.c
	mkdir -p build
	gcc -Wall -g -O0 basic_graph_and_solve.c -o build/a.out -I../include/ -L../target/debug/ -lnavabilitysdk
.PHONY: build-basic-graph

run-basic-graph: build-basic-graph
	@LD_LIBRARY_PATH=../target/debug/ ./build/a.out
.PHONY: run-basic-graph

build-camera-lidar: adding_camera_or_lidar.c
	mkdir -p build
	gcc -Wall -g -O0 adding_camera_or_lidar.c -o build/a.out -I../include/ -L../target/debug/ -lnavabilitysdk
.PHONY: build-camera-lidar

run-camera-lidar: build-camera-lidar
	@LD_LIBRARY_PATH=../target/debug/ ./build/a.out
.PHONY: run-camera-lidar