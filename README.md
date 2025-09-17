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
make fmt            # rustfmt + clippy
make test           # cargo test (all targets with verbose output)
make build          # cargo build (release mode)
make build-release  # cargo build --release
make run            # run the release binary
make clean          # clean build artifacts and caches
make package        # build crate package (allow dirty)
make package-release # build crate package (clean)
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
make package-release # build crate package (clean)
# or use cargo directly:
cargo package --locked
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI builds run automatically on tags matching `v*` and upload the `.crate` file. Uncomment the publish step in `build_package.yml` to automate crates.io releases.

## 🧩 Cross Builds

This template does not ship cross-compile targets by default. If you need cross or zig-based builds, install and configure them per your environment.

GitHub Actions `build_release.yml` builds Linux release binaries on tags matching `v*` and uploads them as release assets.

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

- Before opening a PR, please run locally:
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## 📄 License

MIT — see `LICENSE`.
