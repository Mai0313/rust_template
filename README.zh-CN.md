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
make fmt            # 格式化 + clippy
make test           # 测试
make build          # 调试构建（debug）
make build-release  # 发布构建（release）
make run            # 运行（release）
make clean          # 清理构建产物与缓存
make package        # 构建 crate 包（允许 dirty）
make package-release # 构建 crate 包（clean）
make help           # 查看可用目标
```

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t ghcr.io/<owner>/<repo>:latest .
docker run --rm ghcr.io/<owner>/<repo>:latest
```

或使用实际的二进制名称：
```bash
docker build -f docker/Dockerfile --target prod -t rust_template:latest .
docker run --rm rust_template:latest
```

## 📦 打包发布

```bash
make package        # 构建 crate 包（允许 dirty）
make package-release # 构建 crate 包（clean）
# 或直接使用 cargo：
cargo package --locked
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI 会在打 `v*` 标签时自动打包并上传 `.crate` 产物。若需自动发布 crates.io，请在 `build_package.yml` 打开发布步骤并配置密钥。

## 🧩 跨平台构建

当前模板默认不包含跨编译目标。如需使用 cross 或 zig 进行跨编译，请根据自身环境自行安装与配置。

GitHub Actions `build_release.yml` 会在创建符合 `v*` 的标签时在 Linux 环境构建发布二进制并上传到 Release。

## 🔁 CI/CD

### 主要工作流程
- 测试（`test.yml`）：构建与测试，并输出覆盖率（cargo-llvm-cov）
- 质量（`code-quality-check.yml`）：rustfmt 检查 + clippy（拒绝警告）
- 打包（`build_package.yml`）：标签 `v*` 触发打包，可选 crates.io 发布
- 镜像（`build_image.yml`）：在 `main/master` 与标签 `v*` 推送至 GHCR
- 发布构建（`build_release.yml`）：标签 `v*` 时构建 Linux 发布二进制并上传

### 其他自动化功能
- 自动标签（`auto_labeler.yml`）：根据分支名称与文件变更自动为 PR 添加标签
- 代码扫描（`code_scan.yml`）：安全性扫描
- 发布草稿（`release_drafter.yml`）：自动生成 release notes
- 语义化 PR（`semantic-pull-request.yml`）：检查 PR 标题格式
- Dependabot 每周依赖更新

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


