# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a production-ready Rust project template with comprehensive CI/CD, Docker containerization, and cross-platform build support. The project provides both a library (`src/lib.rs`) and binary (`src/main.rs`) with basic arithmetic functions as examples.

**Key Features:**

- Dynamic version information with git metadata tracking
- Comprehensive CI/CD pipelines for testing, quality checks, and releases
- Multi-platform binary builds (Linux, macOS, Windows for x86_64 and aarch64)
- Docker containerization with multi-stage builds
- Automated dependency updates and security scanning

### Requirements

- **Rust Version**: 1.85 or higher
- **Rust Edition**: 2024 ([Rust 2024 Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2024/index.html))

## Essential Development Commands

### Building and Running

```bash
make build           # Debug build
make build-release   # Release build with optimizations
make run             # Run the release binary
cargo run --release  # Alternative to make run
```

### Testing

```bash
make test            # Run all tests (unit + integration)
make test-verbose    # Run tests with verbose output
cargo test           # Run specific test: cargo test test_name
cargo test --lib     # Run only library tests
cargo test --test basic  # Run specific integration test file
```

### Code Quality

```bash
make fmt             # Format with rustfmt + lint with clippy
cargo fmt --all -- --check        # Check formatting without modifying
cargo clippy --all-targets --all-features -- -D warnings  # Lint with errors on warnings
```

### Coverage

```bash
make coverage        # Generate LCOV coverage report (installs cargo-llvm-cov if needed)
```

### Packaging

```bash
make package         # Build .crate package (allows dirty working directory)
cargo package --locked --allow-dirty
```

### Cleaning

```bash
make clean           # Remove build artifacts, caches, and run git gc
```

## Project Architecture

### Code Structure

- **`src/lib.rs`**: Library code with public API functions (`add`, `multiply`, `subtract`, `calculate_and_display`, `version`, `rust_version`, `cargo_version`)
- **`src/main.rs`**: Binary entry point that uses the library; includes unit tests for main functionality and displays version information on startup
- **`tests/basic.rs`**: Integration tests that verify library API from external consumer perspective
- **`build.rs`**: Build script that generates dynamic version information from git metadata and build environment

### Dynamic Version Information

The project includes automatic version tracking through a build-time script (`build.rs`) that:

- Extracts git metadata (tags, commit count, commit hash, working directory status)
- Captures Rust and Cargo versions used for compilation
- Embeds this information as compile-time environment variables
- Provides public API functions in `src/lib.rs` for accessing version data

**Version Format:** `{version}-{commits}-g{hash}-{dirty}`

Example: `0.1.25-2-gf4ae332-dirty`

- `0.1.25`: Latest git tag (or Cargo.toml version if no tags exist)
- `2`: Number of commits since the tag
- `gf4ae332`: Short commit hash (7 chars)
- `dirty`: Working directory has uncommitted changes

**Available Functions:**

- `version()` - Returns full version string with git metadata
- `rust_version()` - Returns Rust compiler version used for build
- `cargo_version()` - Returns Cargo version used for build

The binary automatically displays this information when executed.

### Key Configuration

- **`Cargo.toml`**: Release profile uses thin LTO, single codegen unit, and strips symbols for optimized binaries
- **`Makefile`**: Provides convenient targets for all common development tasks
- **`docker/Dockerfile`**: Multi-stage build for minimal production images

## Workflow Requirements

### Before Committing

After every edit, run `cargo build` to confirm compilation succeeds. Before committing:

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test`

### Commit Conventions

Use Conventional Commits format for PR titles. Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`.

## CI/CD Workflows

### Main Workflows (triggered on push/PR)

- **`test.yml`**: Runs build and tests, generates LCOV coverage report
- **`code-quality-check.yml`**: Enforces rustfmt and clippy standards (denies warnings)

### Release Workflows (triggered on `v*` tags)

- **`build_package.yml`**: Creates `.crate` package, optional crates.io publish
- **`build_image.yml`**: Builds and pushes Docker image to GHCR
- **`build_release.yml`**: Cross-compiles binaries for 8 platforms, uploads to GitHub Release

### Automation

- **`auto_labeler.yml`**: Labels PRs based on file changes and branch names
- **`code_scan.yml`**: Security scanning (GitLeaks, Trufflehog, CodeQL, Trivy)
- **`release_drafter.yml`**: Auto-generates release notes
- **`semantic-pull-request.yml`**: Validates PR title format
- **Dependabot**: Weekly dependency updates

## Release Process

### Creating a Release

1. Tag with version: `git tag -a v1.0.0 -m "Release v1.0.0"`
2. Push tag: `git push origin v1.0.0`
3. CI automatically builds cross-platform binaries and Docker images

### Supported Platforms

- Linux: `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-gnu`, `aarch64-unknown-linux-musl`
- macOS: `x86_64-apple-darwin`, `aarch64-apple-darwin`
- Windows: `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`

### Asset Naming

- Unix: `rust_template-v{version}-{target}.tar.gz`
- Windows: `rust_template-v{version}-{target}.zip`

## Docker

### Building Images

```bash
docker build -f docker/Dockerfile --target prod -t rust_template:latest .
docker run --rm rust_template:latest
```

### GHCR Publishing

Images automatically pushed to GitHub Container Registry on `main/master` branch and `v*` tags.
