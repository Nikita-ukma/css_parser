# Makefile for Rust project

# Variables
PACKAGE_NAME := css_parser
TARGET_DIR := target/debug
EXECUTABLE := $(TARGET_DIR)/$(PACKAGE_NAME)

# Default target: Build and run the program
.PHONY: run
run: build
	@echo "Running the program..."
	$(EXECUTABLE)

# Build the project
.PHONY: build
build:
	@echo "Building the project..."
	cargo build

# Run tests
.PHONY: test
test:
	@echo "Running tests..."
	cargo test

# Run Clippy for linting
.PHONY: lint
lint:
	@echo "Running Clippy for linting..."
	cargo clippy -- -D warnings

# Format code
.PHONY: fmt
fmt:
	@echo "Formatting code..."
	cargo fmt

# Check formatting, linting, and run tests before committing
.PHONY: check
check: fmt lint test
	@echo "All checks passed!"

# Clean the project
.PHONY: clean
clean:
	@echo "Cleaning the project..."
	cargo clean

# Run all the above in one command
.PHONY: all
all: clean fmt lint build test run

# Help: List available commands
.PHONY: help
help:
	@echo "Available commands:"
	@echo "  make run       - Build and run the program"
	@echo "  make build     - Build the project"
	@echo "  make test      - Run tests"
	@echo "  make lint      - Run Clippy for linting"
	@echo "  make fmt       - Format the code"
	@echo "  make check     - Run format, lint, and test"
	@echo "  make clean     - Clean the project"
	@echo "  make all       - Clean, format, lint, build, test, and run the project"