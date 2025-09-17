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
	cargo build --release --locked

package: ## Build crate package (.crate)
	cargo package --locked --allow-dirty

test: ## Run all tests
	cargo test --all --verbose

run: ## Run the application
	cargo run --release

# -----------------------------------------------------------------------------
# Cross-compilation utilities
# -----------------------------------------------------------------------------

print-targets: ## Print default target lists
	@echo "LINUX_GNU_TARGETS    = $(LINUX_GNU_TARGETS)"
	@echo "LINUX_MUSL_TARGETS   = $(LINUX_MUSL_TARGETS)"
	@echo "WINDOWS_GNU_TARGETS  = $(WINDOWS_GNU_TARGETS)"
	@echo "WINDOWS_MSVC_TARGETS = $(WINDOWS_MSVC_TARGETS)"
	@echo "APPLE_TARGETS        = $(APPLE_TARGETS)"
	@echo "WASM_TARGETS         = $(WASM_TARGETS)"
	@echo "ALL_TARGETS          = $(ALL_TARGETS)"

install-targets: ## Install Rust targets via rustup (set TARGETS="...")
	rustup target add $(TARGETS)

install-cross: ## Install cross for Docker-based cross compilation
	cargo install cross --git https://github.com/cross-rs/cross

install-zigbuild: ## Install cargo-zigbuild for Apple cross-compilation
	cargo install cargo-zigbuild

setup-ubuntu: ## Install common cross toolchains on Ubuntu (requires sudo)
	sudo apt-get update
	sudo apt-get install -y --no-install-recommends build-essential pkg-config cmake ninja-build musl-tools mingw-w64 clang lld zip

build-targets: ## Build release for TARGETS (set CROSS=1 to use cross)
	@[ -n "$(TARGETS)" ] || (echo "TARGETS is empty" && exit 1)
	set -e; \
	for target in $(TARGETS); do \
		printf "\n==> Building for %s (CROSS=%s)\n" "$$target" "$(CROSS)"; \
		$(BUILD_TOOL) build --target $$target --release --locked; \
	done

build-targets-zig: ## Build release for TARGETS using cargo-zigbuild (Apple)
	@[ -n "$(TARGETS)" ] || (echo "TARGETS is empty" && exit 1)
	set -e; \
	for target in $(TARGETS); do \
		printf "\n==> Zig building for %s\n" "$$target"; \
		cargo zigbuild --target $$target --release --locked; \
	done

dist: ## Collect built binaries into dist/ and archive per-target
	rm -rf dist && mkdir -p dist
	set -e; \
	for target in $(TARGETS); do \
		OUT_DIR="dist/$$target"; \
		mkdir -p "$$OUT_DIR"; \
		BIN_PATH="target/$$target/release/$(BIN_NAME)"; \
		if echo "$$target" | grep -qi windows; then BIN_PATH="$$BIN_PATH.exe"; fi; \
		if [ ! -f "$$BIN_PATH" ]; then echo "Missing binary: $$BIN_PATH" && exit 1; fi; \
		cp "$$BIN_PATH" "$$OUT_DIR/"; \
		if echo "$$target" | grep -qi windows; then \
			( cd dist && zip -9 -r "$(BIN_NAME)-$$target.zip" "$$target" ); \
		else \
			tar czf "dist/$(BIN_NAME)-$$target.tar.gz" -C dist "$$target"; \
		fi; \
	done

dist-native: ## Package the natively-built binary in dist/
	rm -rf dist && mkdir -p dist/native
	BIN_PATH="target/release/$(BIN_NAME)"; \
	if [ -f "$$BIN_PATH.exe" ]; then BIN_PATH="$$BIN_PATH.exe"; fi; \
	if [ ! -f "$$BIN_PATH" ]; then echo "Missing native binary: $$BIN_PATH" && exit 1; fi; \
	cp "$$BIN_PATH" dist/native/
	if echo "$$BIN_PATH" | grep -qi ".exe$"; then \
		( cd dist && zip -9 -r "$(BIN_NAME)-native.zip" native ); \
	else \
		tar czf "dist/$(BIN_NAME)-native.tar.gz" -C dist native; \
	fi

build-all: ## Build and package all mainstream targets (no Docker) — may require toolchains
	$(MAKE) install-targets TARGETS="$(ALL_TARGETS)"
	$(MAKE) build-targets TARGETS="$(ALL_TARGETS)" CROSS=0
	$(MAKE) dist TARGETS="$(ALL_TARGETS)"

cross-all: ## Build all mainstream targets using cross (Linux host recommended)
	$(MAKE) build-targets TARGETS="$(ALL_TARGETS)" CROSS=1
	$(MAKE) dist TARGETS="$(ALL_TARGETS)"

release: ## Build and package all targets for release (cross-compilation + Apple via zigbuild)
	@echo "==> Installing cross-compilation tools..."
	cargo install cross --git https://github.com/cross-rs/cross || true
	cargo install cargo-zigbuild || true
	@echo "==> Building Linux/Windows/WASM targets via cross..."
	$(MAKE) build-targets CROSS=1 TARGETS="$(LINUX_GNU_TARGETS) $(LINUX_MUSL_TARGETS) $(WINDOWS_GNU_TARGETS) $(WASM_TARGETS)"
	@echo "==> Building Apple targets via cargo-zigbuild..."
	rustup target add $(APPLE_TARGETS) || true
	$(MAKE) build-targets-zig TARGETS="$(APPLE_TARGETS)" || echo "Apple build failed, continuing..."
	@echo "==> Packaging all built binaries..."
	$(MAKE) dist TARGETS="$(ALL_TARGETS)"
	@echo "==> Release build complete! Check dist/ directory."
