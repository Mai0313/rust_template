<center>

# Rust Project Template

[![rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![tests](https://github.com/Mai0313/rust_template/actions/workflows/test.yml/badge.svg)](https://github.com/Mai0313/rust_template/actions/workflows/test.yml)
[![code-quality](https://github.com/Mai0313/rust_template/actions/workflows/code-quality-check.yml/badge.svg)](https://github.com/Mai0313/rust_template/actions/workflows/code-quality-check.yml)
[![license](https://img.shields.io/badge/License-MIT-green.svg?labelColor=gray)](https://github.com/Mai0313/rust_template/tree/master?tab=License-1-ov-file)
[![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/Mai0313/rust_template/pulls)
[![contributors](https://img.shields.io/github/contributors/Mai0313/rust_template.svg)](https://github.com/Mai0313/rust_template/graphs/contributors)

</center>

🚀 A production‑ready Rust project template to bootstrap new projects fast. It includes a clean Cargo layout, Docker, and a complete CI/CD suite.

Click [Use this template](https://github.com/Mai0313/rust_template/generate) to start a new repository from this scaffold.

Other Languages: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## ✨ Highlights

- Modern Cargo layout (`src/lib.rs`, `src/main.rs`, `tests/`)
- Lint & format with clippy and rustfmt
- GitHub Actions: tests, quality, package build, Docker publish, release drafter, Rust-aware labeler, secret scans, semantic PR, weekly dependency update
- Multi-stage Dockerfile producing a minimal runtime image

## 🚀 Quick Start

Prerequisites: Rust toolchain (`rustup`), Docker (optional)

```bash
make format    # rustfmt
make lint      # clippy -D warnings
make test      # cargo test
make build     # cargo build --release
make run       # run the binary
```

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t ghcr.io/<owner>/<repo>:latest .
docker run --rm ghcr.io/<owner>/<repo>:latest
```

## 📦 Packaging

```bash
cargo package
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI builds run automatically on tags matching `v*` and upload the `.crate` file. Uncomment the publish step in `build_package.yml` to automate crates.io releases.

## 🧩 Cross Builds

Using the Makefile you can build for multiple target triples (actions call the same targets):

```bash
# Linux/Windows/WASM on Ubuntu via cross
cargo install cross --git https://github.com/cross-rs/cross
make build-targets CROSS=1 TARGETS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu wasm32-wasi"
make dist TARGETS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu wasm32-wasi"

# macOS on Ubuntu via zig + cargo-zigbuild (requires zig)
cargo install cargo-zigbuild
# install zig (e.g. with your package manager)
make build-targets-zig TARGETS="x86_64-apple-darwin aarch64-apple-darwin"
make dist TARGETS="x86_64-apple-darwin aarch64-apple-darwin"
```

GitHub Actions `build_release.yml` cross-builds these artifacts on Ubuntu and uploads them as release assets on tags.

## 🔁 CI/CD Workflows

- Tests (`test.yml`): cargo build/test + coverage artifact
- Code Quality (`code-quality-check.yml`): rustfmt check + clippy (deny warnings)
- Build Package (`build_package.yml`): package on tag `v*`, optional crates.io publish
- Publish Docker Image (`build_image.yml`): push to GHCR on `master` and tags `v*`
- Build Release (`build_release.yml`): cross-compiled archives for Linux, Windows, macOS, WASM
- Release Drafter, Labeler, Secret Scanning, Semantic PR, Weekly cargo update

## 🤝 Contributing

- Open issues/PRs
- Use Conventional Commits for PR titles
- Keep code formatted and clippy‑clean

- Before opening a PR, please run locally:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## 📄 License

MIT — see `LICENSE`.
