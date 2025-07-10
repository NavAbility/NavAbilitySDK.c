

INC_DIR = ../include/
LIB_DIR = ../target/debug/

CXX := gcc
CXXFLAGS = -Wall -g -fwrapv -I$(INC_DIR)

.DEFAULT_GOAL := run-all

clean:
	@echo "Cleaning up build files..."
	rm -rf build
.PHONY: clean

run-all: run-basic-graph run-camera-lidar run-agents-graphs run-oversimplified-localizer
	@echo "All examples have been run successfully."
.PHONY: run-all

build-all: build-basic-graph build-camera-lidar build-agents-graphs build-oversimplified-localizer
	@echo "All examples have been built successfully."
.PHONY: build-all

build-basic-graph: basic_graph_and_solve.c
	mkdir -p build
	$(CXX) $(CXXFLAGS) basic_graph_and_solve.c -o build/basic_graph -L$(LIB_DIR) -lnavabilitysdk
.PHONY: build-basic-graph

run-basic-graph: build-basic-graph
	@LD_LIBRARY_PATH=../target/debug/ RUST_BACKTRACE=full ./build/basic_graph
.PHONY: run-basic-graph

build-camera-lidar: adding_camera_or_lidar.c
	mkdir -p build
	$(CXX) $(CXXFLAGS) adding_camera_or_lidar.c -o build/camera_lidar_data -L$(LIB_DIR) -lnavabilitysdk
.PHONY: build-camera-lidar

run-camera-lidar: build-camera-lidar
	@LD_LIBRARY_PATH=../target/debug/ RUST_BACKTRACE=full ./build/camera_lidar_data
.PHONY: run-camera-lidar

build-agents-graphs: retrieve_agents_graphs_estimates.c
	mkdir -p build
	$(CXX) $(CXXFLAGS) retrieve_agents_graphs_estimates.c -o build/agents_graphs -L$(LIB_DIR) -lnavabilitysdk
.PHONY: build-agents-graphs

run-agents-graphs: build-agents-graphs
	@LD_LIBRARY_PATH=../target/debug/ RUST_BACKTRACE=full ./build/agents_graphs
.PHONY: run-agents-graphs

build-oversimplified-localizer: oversimplified_nontree_localizer.c
	mkdir -p build
	$(CXX) $(CXXFLAGS) oversimplified_nontree_localizer.c -o build/oversimple_localizer -L$(LIB_DIR) -lnavabilitysdk
.PHONY: build-oversimplified-localizer

run-oversimplified-localizer: build-oversimplified-localizer
	@LD_LIBRARY_PATH=../target/debug/ RUST_BACKTRACE=full ./build/oversimple_localizer
.PHONY: run-oversimplified-localizer