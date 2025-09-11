<!-- Workspace-specific instructions for GitHub Copilot. Keep this in sync with the repo. -->

⚠️ IMPORTANT: Update this document whenever tooling, commands, or workflows change.

### Project Background

This repository is a Rust project template intended to help developers bootstrap projects quickly. It ships with a modern Cargo layout, Docker multi-stage builds, and a comprehensive CI/CD setup via GitHub Actions.

### Core Infrastructure

- Rust toolchain: stable (via rustup)
- Dependency and build: `cargo`
- Lint/format: `clippy` and `rustfmt`
- Dockerfile with multi-stage builds targeting a minimal runtime image

### Local Development

- Format: `make format` (runs `cargo fmt --all`)
- Format (check): `make format-check`
- Lint: `make lint` (runs `cargo clippy -- -D warnings`)
- Quality: `make quality` (format-check + clippy)
- Test: `make test` (runs `cargo test --all`)
- Build: `make build` (release build)
- Run: `make run` (runs the binary)
- Coverage: `make coverage` (generates `lcov.info` with cargo-llvm-cov)

Make targets reference (from `Makefile`):

- `make clean`: remove `target/` and caches
- `make format`: rustfmt
- `make format-check`: rustfmt --check
- `make lint`: clippy with denied warnings
- `make quality`: format-check + clippy
- `make test`: cargo test
- `make build`: cargo build --release
- `make run`: cargo run --release
- `make package`: cargo package
- `make submodule-init|submodule-update`: submodule helpers (optional)

Cross-compilation & packaging helpers:

- Target sets (Tier1/2 mainstream):
  - Linux GNU: `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `i686-unknown-linux-gnu`, `armv7-unknown-linux-gnueabihf`
  - Linux MUSL: `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, `i686-unknown-linux-musl`, `armv7-unknown-linux-musleabihf`
  - Windows GNU: `x86_64-pc-windows-gnu`, `aarch64-pc-windows-gnu`, `i686-pc-windows-gnu`
  - Windows MSVC: `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc`
  - Apple Darwin: `x86_64-apple-darwin`, `aarch64-apple-darwin`
  - WASM: `wasm32-wasi`

- Build selected targets (native toolchains): `make build-targets TARGETS="<space-separated-targets>"`
- Build selected targets with cross: `make build-targets CROSS=1 TARGETS="..."`
- Build Apple targets with zig: `make build-targets-zig TARGETS="x86_64-apple-darwin aarch64-apple-darwin"`
- Collect artifacts: `make dist TARGETS="..."` or `make dist-native`

Notes:
- On Ubuntu, Linux/Windows(GNU)/WASM builds can be done via `cross` (Docker-based)
- Apple (Darwin) builds on Ubuntu require `zig` + `cargo-zigbuild` and may require additional SDK pieces depending on your dependencies

### Crate Usage

- Library API lives in `src/lib.rs`
- Binary entrypoint lives in `src/main.rs`
- Run locally: `cargo run --release`

### Coding Style

- Use rustfmt for formatting
- Use clippy and treat warnings as errors in CI
- Prefer clear, explicit types on public APIs

### Documentation

- Generate docs: `cargo doc --no-deps`
- Optionally view locally: `cargo doc --open`

### Dependencies (Cargo)

- Add dependency: `cargo add <crate>`
- Update lockfile: `cargo update`

Build and publish:

```bash
cargo package                      # create .crate in target/package/
CARGO_REGISTRY_TOKEN=... cargo publish   # publish to crates.io
```

### Docker

- Multi-stage Dockerfile at `docker/Dockerfile`
- Build prod image: `docker build -f docker/Dockerfile --target prod -t <image> .`

### CI/CD Workflows (GitHub Actions)

All workflows live in `.github/workflows/`:

- `test.yml`: Set up Rust toolchain, cargo build/test, upload coverage (cargo-llvm-cov)
- `code-quality-check.yml`: Run rustfmt (check) and clippy (deny warnings)
- `build_package.yml`: On tags `v*`, bump version from tag, `cargo package`; upload `.crate`, optional crates.io publish
- `build_image.yml`: Build and push Docker image to GHCR on `master` and tags `v*`
- `build_release.yml`: On tags, cross-builds Linux/Windows(GNU)/WASM via `cross` and Apple via `cargo-zigbuild` on Ubuntu; uploads archives and attaches to release
- `release_drafter.yml`: Maintain a draft release using Conventional Commits
- `auto_labeler.yml`: Auto-apply labels based on `.github/labeler.yml` (configured with Rust file patterns)
- `code_scan.yml`: Secret scan (gitleaks/trufflehog), CodeQL (Rust), Trivy filesystem scan
- `semantic-pull-request.yml`: Enforce Conventional Commit PR titles
- `pre-commit-updater.yml` (repurposed): Weekly `cargo update` to refresh `Cargo.lock` and open a PR

### Conventions

- Conventional Commit PR titles (enforced by workflow)
- Prefer small, focused PRs with tests and docs
- Update this file plus `README.md`, `README.zh-TW.md`, and `README.zh-CN.md` when workflows, commands, or usage change

- Before opening a PR locally run: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
