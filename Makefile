.PHONY: all

all: build ## Default target: build release binary

help: # Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

clean: ## Clean build artifacts and caches
	@rm -rf target dist tmp .cache
	@find . -type f -name "*.DS_Store" -ls -delete
	@cargo install cargo-cache --quiet
	@cargo cache --autoclean
	@git fetch --prune
	@git gc --prune=now --aggressive

fmt: ## Format code with rustfmt and Lint with clippy
	cargo fmt --all
	cargo clippy --all-targets --all-features

build: ## Build release binary
	cargo build

build-release: ## Build release binary
	cargo build --release --locked

package: ## Build crate package (.crate)
	cargo package --locked --allow-dirty

package-release: ## Build crate package (.crate)
	cargo package --locked

test: ## Run all tests
	cargo test --all --verbose

run: ## Run the application
	cargo run --release
