# Project metadata
BIN_NAME ?= rust_template
CARGO ?= cargo
# When CROSS=1, use `cross` for builds (https://github.com/cross-rs/cross)
CROSS ?= 0
BUILD_TOOL := $(if $(filter 1,$(CROSS)),cross,$(CARGO))

# Common target triples (Tier 1/2 mainstream)
LINUX_GNU_TARGETS := x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu i686-unknown-linux-gnu armv7-unknown-linux-gnueabihf
LINUX_MUSL_TARGETS := x86_64-unknown-linux-musl aarch64-unknown-linux-musl i686-unknown-linux-musl armv7-unknown-linux-musleabihf
WINDOWS_GNU_TARGETS := x86_64-pc-windows-gnu i686-pc-windows-gnu
WINDOWS_MSVC_TARGETS := x86_64-pc-windows-msvc aarch64-pc-windows-msvc
APPLE_TARGETS := x86_64-apple-darwin aarch64-apple-darwin
WASM_TARGETS := wasm32-wasi

ALL_TARGETS := $(LINUX_GNU_TARGETS) $(LINUX_MUSL_TARGETS) $(WINDOWS_GNU_TARGETS) $(WINDOWS_MSVC_TARGETS) $(APPLE_TARGETS) $(WASM_TARGETS)
TARGETS ?= $(ALL_TARGETS)

.PHONY: all

all: build ## Default target: build release binary

help: # Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean build artifacts and caches
	@rm -rf target dist tmp .cache
	@find . -type f -name "*.DS_Store" -ls -delete
	@$(GO) clean -cache
	@$(GO) clean -testcache
	@$(GO) clean -fuzzcache
	@git fetch --prune
	@git gc --prune=now --aggressive

format: ## Format code with rustfmt and Lint with clippy
	cargo fmt --all
	cargo clippy --all-targets --all-features

build: ## Build release binary
	cargo build

build-release: ## Build release binary
	cargo build --release --locked

package: ## Build crate package (.crate)
	cargo package --locked --allow-dirty

test: ## Run all tests
	cargo test --all --verbose

run: ## Run the application
	cargo run --release
