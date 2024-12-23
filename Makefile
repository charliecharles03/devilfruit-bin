# Variables
CARGO = cargo
BUILD_DIR = target

# Default target
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	@echo "Building the Rust project..."
	$(CARGO) build

# Run the project
.PHONY: run
run:
	@echo "Running the Rust project..."
	$(CARGO) run

# Clean the build artifacts
.PHONY: clean
clean:
	@echo "Cleaning up build artifacts..."
	$(CARGO) clean

# Test the project
.PHONY: test
test:
	@echo "Running tests..."
	$(CARGO) test

