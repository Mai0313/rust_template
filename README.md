<center>

# Rust Project Template

[![Crates.io](https://img.shields.io/crates/v/rust_template?logo=rust&style=flat-square&color=E05D44)](https://crates.io/crates/rust_template)
[![Crates.io Downloads](https://img.shields.io/crates/d/rust_template?logo=rust&style=flat-square)](https://crates.io/crates/rust_template)
[![npm version](https://img.shields.io/npm/v/rust_template?logo=npm&style=flat-square&color=CB3837)](https://www.npmjs.com/package/rust_template)
[![npm downloads](https://img.shields.io/npm/dt/rust_template?logo=npm&style=flat-square)](https://www.npmjs.com/package/rust_template)
[![rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white&style=flat-square)](https://www.rust-lang.org/)
[![tests](https://img.shields.io/github/actions/workflow/status/Mai0313/rust_template/test.yml?label=tests&logo=github&style=flat-square)](https://github.com/Mai0313/rust_template/actions/workflows/test.yml)
[![code-quality](https://img.shields.io/github/actions/workflow/status/Mai0313/rust_template/code-quality-check.yml?label=code-quality&logo=github&style=flat-square)](https://github.com/Mai0313/rust_template/actions/workflows/code-quality-check.yml)
[![license](https://img.shields.io/badge/License-MIT-green.svg?labelColor=gray&style=flat-square)](https://github.com/Mai0313/rust_template/tree/master?tab=License-1-ov-file)
[![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/Mai0313/rust_template/pulls)

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
make fmt            # rustfmt + clippy
make test           # cargo test (all targets)
make test-verbose   # cargo test (all targets with verbose output)
make coverage       # generate LCOV coverage report
make build          # cargo build (release mode)
make build-release  # cargo build --release
make run            # run the release binary
make clean          # clean build artifacts and caches
make package        # build crate package (allow dirty)
make help           # list targets
```

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t ghcr.io/<owner>/<repo>:latest .
docker run --rm ghcr.io/<owner>/<repo>:latest
```

Or using the actual binary name:

```bash
docker build -f docker/Dockerfile --target prod -t rust_template:latest .
docker run --rm rust_template:latest
```

## 📦 Packaging

```bash
make package        # build crate package (allow dirty)
# or use cargo directly:
cargo package --locked --allow-dirty
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI builds run automatically on tags matching `v*` and upload the `.crate` file. Uncomment the publish step in `build_package.yml` to automate crates.io releases.

## 🧩 Cross Builds

This template does not ship cross-compile tooling by default. If you need cross or zig-based builds locally, install and configure them per your environment.

GitHub Actions `build_release.yml` builds multi-platform release binaries on tags matching `v*` and uploads them to the GitHub Release assets.

Targets:

- x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu, aarch64-unknown-linux-musl
- x86_64-apple-darwin, aarch64-apple-darwin
- x86_64-pc-windows-msvc, aarch64-pc-windows-msvc

Assets naming:

- `<bin>-v<version>-<target>.tar.gz` (all platforms)
- `<bin>-v<version>-<target>.zip` (Windows additionally)

## 🔁 CI/CD Workflows

### Main Workflows

- Tests (`test.yml`): cargo build/test + generate LCOV coverage report and upload artifact
- Code Quality (`code-quality-check.yml`): rustfmt check + clippy (deny warnings)
- Build Package (`build_package.yml`): package on tag `v*`, optional crates.io publish
- Publish Docker Image (`build_image.yml`): push to GHCR on `main/master` and tags `v*`
- Build Release (`build_release.yml`): Linux release binaries uploaded on tags `v*`

### Additional Automation

- Auto Labeler (`auto_labeler.yml`): automatically label PRs based on branch names and file changes
- Code Scan (`code_scan.yml`): multi-layer security scanning (GitLeaks, Trufflehog secret scanning, CodeQL code analysis, Trivy vulnerability scanning)
- Release Drafter (`release_drafter.yml`): auto-generate release notes
- Semantic PR (`semantic-pull-request.yml`): enforce PR title format
- Dependabot weekly dependency updates

## 🤝 Contributing

- Open issues/PRs

- Use Conventional Commits for PR titles

- Keep code formatted and clippy‑clean

- After every edit, run `cargo build` to confirm compilation is successful

- Before opening a PR, please run locally:

  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## 📄 License

MIT — see `LICENSE`.
