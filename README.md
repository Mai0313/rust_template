<div align="center" markdown="1">

# Rust Project Template

[![Crates.io](https://img.shields.io/crates/v/rust_template?logo=rust&style=flat-square&color=E05D44)](https://crates.io/crates/rust_template)
[![Crates.io Downloads](https://img.shields.io/crates/d/rust_template?logo=rust&style=flat-square)](https://crates.io/crates/rust_template)
[![npm version](https://img.shields.io/npm/v/rust_template?logo=npm&style=flat-square&color=CB3837)](https://www.npmjs.com/package/rust_template)
[![npm downloads](https://img.shields.io/npm/dt/rust_template?logo=npm&style=flat-square)](https://www.npmjs.com/package/rust_template)
[![PyPI version](https://img.shields.io/pypi/v/rust_template?logo=python&style=flat-square&color=3776AB)](https://pypi.org/project/rust_template/)
[![PyPI downloads](https://img.shields.io/pypi/dm/rust_template?logo=python&style=flat-square)](https://pypi.org/project/rust_template/)
[![rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white&style=flat-square)](https://www.rust-lang.org/)
[![tests](https://img.shields.io/github/actions/workflow/status/Mai0313/rust_template/test.yml?label=tests&logo=github&style=flat-square)](https://github.com/Mai0313/rust_template/actions/workflows/test.yml)
[![code-quality](https://img.shields.io/github/actions/workflow/status/Mai0313/rust_template/code-quality-check.yml?label=code-quality&logo=github&style=flat-square)](https://github.com/Mai0313/rust_template/actions/workflows/code-quality-check.yml)
[![license](https://img.shields.io/badge/License-MIT-green.svg?labelColor=gray&style=flat-square)](https://github.com/Mai0313/rust_template/tree/master?tab=License-1-ov-file)
[![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/Mai0313/rust_template/pulls)

</div>

🚀 A production‑ready Rust project template to bootstrap new projects fast. It includes a clean Cargo layout, Docker, and a complete CI/CD suite.

Click [Use this template](https://github.com/Mai0313/rust_template/generate) to start a new repository from this scaffold.

Other Languages: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## 🎯 Using This Template

**IMPORTANT**: This is a template repository. Before using it for your project, you must:

1. **Rename all occurrences** of `rust_template` to your project name across the entire codebase
2. **Update metadata** in `Cargo.toml`, `cli/nodejs/package.json`, and `cli/python/pyproject.toml`
3. **Update author information** in all package manifests and Dockerfiles
4. **Update repository URLs** in README badges, package manifests, and GitHub workflows
5. **Rename the Python package directory** from `cli/python/src/rust_template` to your project name

For detailed step-by-step instructions, see [CLAUDE.md](CLAUDE.md).

**Quick verification after setup**:

```bash
grep -r "rust_template" . --exclude-dir=target --exclude-dir=.git  # Should find minimal matches
make fmt && cargo build && cargo test --all  # Verify everything works
```

## ✨ Highlights

- Modern Cargo layout with unit tests in `src/` and integration tests in `tests/`
- Dynamic version information with git metadata (tag, commit hash, build tools)
- Lint & format with clippy and rustfmt
- GitHub Actions: tests, quality, package build, Docker publish, release drafter, Rust-aware labeler, secret scans, semantic PR, daily dependency update
- Multi-stage Dockerfile producing a minimal runtime image

## 📌 Version Information

The binary automatically displays dynamic version information including:

- Git tag version (or `Cargo.toml` version if no tags)
- Commit count since last tag
- Short commit hash
- Dirty working directory indicator
- Rust and Cargo versions used for building

Example output:

```
rust_template v0.1.25-2-gf4ae332-dirty
Built with Rust 1.90.0 and Cargo 1.90.0
```

This version information is embedded at build time through `build.rs` and automatically updated based on your git state.

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

## 🛠️ Development

Contributor setup, commands, tests, code conventions, CI and releases live in [CONTRIBUTING.md](./.github/CONTRIBUTING.md).

## 📄 License

MIT — see `LICENSE`.
