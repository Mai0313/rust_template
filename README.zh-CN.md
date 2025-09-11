<center>

# Rust 项目模板

[![rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![tests](https://github.com/Mai0313/rust_template/actions/workflows/test.yml/badge.svg)](https://github.com/Mai0313/rust_template/actions/workflows/test.yml)
[![code-quality](https://github.com/Mai0313/rust_template/actions/workflows/code-quality-check.yml/badge.svg)](https://github.com/Mai0313/rust_template/actions/workflows/code-quality-check.yml)
[![license](https://img.shields.io/badge/License-MIT-green.svg?labelColor=gray)](https://github.com/Mai0313/rust_template/tree/master?tab=License-1-ov-file)
[![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://github.com/Mai0313/rust_template/pulls)

</center>

🚀 帮助 Rust 开发者「快速建立新项目」的模板。内置 Cargo 布局、Docker 与完整 CI/CD 工作流。

点击 [使用此模板](https://github.com/Mai0313/rust_template/generate) 后即可开始。

其他语言: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## ✨ 特色

- 现代 Cargo 结构（`src/lib.rs`、`src/main.rs`、`tests/`）
- clippy + rustfmt 质量保障
- GitHub Actions：测试、质量、打包、Docker 推送、发布草稿、Rust 自动加标签、秘密扫描、语义化 PR、每周依赖更新
- 多阶段 Dockerfile，产出精简运行镜像

## 🚀 快速开始

前置：安装 Rust（rustup）、可选 Docker

```bash
make format    # 格式化
make lint      # clippy（将警告视为错误）
make test      # 测试
make build     # 发布构建
make run       # 运行
```

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t ghcr.io/<owner>/<repo>:latest .
docker run --rm ghcr.io/<owner>/<repo>:latest
```

## 📦 打包发布

```bash
cargo package
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI 会在打 `v*` 标签时自动打包并上传 `.crate` 产物。若需自动发布 crates.io，请在 `build_package.yml` 打开发布步骤并配置密钥。

## 🧩 跨平台构建

通过 Makefile 可构建多种目标（三元组），Actions 也会调用相同目标：

```bash
# 在 Ubuntu 使用 cross 构建 Linux/Windows/WASM
cargo install cross --git https://github.com/cross-rs/cross
make build-targets CROSS=1 TARGETS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu wasm32-wasi"
make dist TARGETS="x86_64-unknown-linux-gnu x86_64-pc-windows-gnu wasm32-wasi"

# 在 Ubuntu 使用 zig + cargo-zigbuild 构建 macOS（需安装 zig）
cargo install cargo-zigbuild
# 安装 zig（按环境选择方式）
make build-targets-zig TARGETS="x86_64-apple-darwin aarch64-apple-darwin"
make dist TARGETS="x86_64-apple-darwin aarch64-apple-darwin"
```

GitHub Actions `build_release.yml` 会在 Ubuntu 上执行以上交叉编译，并在打标签时上传到 Release。

## 🔁 CI/CD

- 测试（`test.yml`）：构建/测试 + 覆盖率产物
- 质量（`code-quality-check.yml`）：rustfmt 检查 + clippy（拒绝警告）
- 打包（`build_package.yml`）：标签触发打包，可选 crates.io 发布
- 镜像（`build_image.yml`）：推送 GHCR
- 发布构建（`build_release.yml`）：Linux/Windows/macOS/WASM 跨编译封装
- 另含 Release Drafter、Labeler、Secret Scanning、Semantic PR、每周 cargo update

## 🤝 贡献

- 欢迎 Issue/PR
- PR 标题遵循 Conventional Commits
- 保持格式化并通过 clippy 检查

- 在提交 PR 前，请先本地执行：
  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## 📄 授权

MIT — 详见 `LICENSE`。


