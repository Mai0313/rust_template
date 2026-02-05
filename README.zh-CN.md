<div align="center" markdown="1">

# Rust 项目模板

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

🚀 帮助 Rust 开发者「快速建立新项目」的模板。内置 Cargo 布局、Docker 与完整 CI/CD 工作流。

点击 [使用此模板](https://github.com/Mai0313/rust_template/generate) 后即可开始。

其他语言: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## 🎯 使用此模板

**重要提示**：这是一个模板仓库。在将其用于您的项目之前，您必须：

1. **重命名所有出现的** `rust_template` 为您的项目名称（整个代码库）
2. **更新元数据**：修改 `Cargo.toml`、`cli/nodejs/package.json` 和 `cli/python/pyproject.toml`
3. **更新作者信息**：修改所有包清单和 Dockerfile 中的作者信息
4. **更新仓库 URL**：修改 README 徽章、包清单和 GitHub workflows 中的链接
5. **重命名 Python 包目录**：将 `cli/python/src/rust_template` 改为您的项目名称

详细的分步说明请参阅 [.github/copilot-instructions.md](.github/copilot-instructions.md#using-this-template-for-new-projects)。

**设置后的快速验证**：
```bash
grep -r "rust_template" . --exclude-dir=target --exclude-dir=.git  # 应该只找到少量匹配
make fmt && cargo build && cargo test --all  # 验证一切正常
```

## ✨ 特色

- 现代 Cargo 结构（`src/lib.rs`、`src/main.rs`、`tests/`）
- 动态版本信息，包含 git 元数据（标签、提交哈希、构建工具）
- clippy + rustfmt 质量保障
- GitHub Actions：测试、质量、打包、Docker 推送、发布草稿、Rust 自动加标签、秘密扫描、语义化 PR、每周依赖更新
- 多阶段 Dockerfile，产出精简运行镜像

## 🚀 快速开始

**系统要求：**

- Rust 1.85 或更高版本（使用 Edition 2024）
- Docker（可选）

如尚未安装 Rust，请使用 `rustup` 进行安装。

```bash
make fmt            # 格式化 + clippy
make test           # 测试（所有目标）
make test-verbose   # 测试（所有目标与详细输出）
make coverage       # 生成 LCOV 覆盖率报告
make build          # 构建（release 模式）
make build-release  # 发布构建（release）
make run            # 运行（release）
make clean          # 清理构建产物与缓存
make package        # 构建 crate 包（允许 dirty）
make help           # 查看可用目标
```

## 📌 版本信息

可执行文件会自动显示动态版本信息，包含：

- Git 标签版本（若无标签则使用 `Cargo.toml` 版本）
- 自上次标签以来的提交数量
- 简短提交哈希值
- 工作目录是否有未提交的更改（dirty 标记）
- 构建时使用的 Rust 与 Cargo 版本

输出示例：

```
rust_template v0.1.25-2-gf4ae332-dirty
Built with Rust 1.90.0 and Cargo 1.90.0
```

这些版本信息会在构建时通过 `build.rs` 自动嵌入，并根据您的 git 状态动态更新。

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
# 或直接使用 cargo：
cargo package --locked --allow-dirty
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI 会在打 `v*` 标签时自动打包并上传 `.crate` 产物。若需自动发布 crates.io，请在 `build_package.yml` 打开发布步骤并配置密钥。

## 🧩 跨平台构建

当前模板默认不包含本地跨编译工具。如需在本地使用 cross 或 zig，请按需安装与配置。

GitHub Actions `build_release.yml` 会在创建 `v*` 标签时为多平台构建发布二进制，并上传到 GitHub Release。

目标（targets）：

- x86_64-unknown-linux-gnu、x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu、aarch64-unknown-linux-musl
- x86_64-apple-darwin、aarch64-apple-darwin
- x86_64-pc-windows-msvc、aarch64-pc-windows-msvc

资产命名（assets）：

- `<bin>-v<version>-<target>.tar.gz`（所有平台）
- `<bin>-v<version>-<target>.zip`（Windows 额外提供）

## 🔁 CI/CD

### 主要工作流程

- 测试（`test.yml`）：构建与测试，生成 LCOV 格式覆盖率报告并上传 artifact
- 质量（`code-quality-check.yml`）：rustfmt 检查 + clippy（拒绝警告）
- 打包（`build_package.yml`）：标签 `v*` 触发打包，可选 crates.io 发布
- 镜像（`build_image.yml`）：在 `main/master` 与标签 `v*` 推送至 GHCR
- 发布构建（`build_release.yml`）：标签 `v*` 时构建 Linux 发布二进制并上传

### 其他自动化功能

- 自动标签（`auto_labeler.yml`）：根据分支名称与文件变更自动为 PR 添加标签
- 代码扫描（`code_scan.yml`）：多层安全性扫描（GitLeaks、Trufflehog 秘密扫描、CodeQL 代码分析、Trivy 漏洞扫描）
- 发布草稿（`release_drafter.yml`）：自动生成 release notes
- 语义化 PR（`semantic-pull-request.yml`）：检查 PR 标题格式
- Dependabot 每周依赖更新

## 🤝 贡献

- 欢迎 Issue/PR

- PR 标题遵循 Conventional Commits

- 保持格式化并通过 clippy 检查

- 每次编辑完毕后，请执行 `cargo build` 来确认编译是否成功

- 在提交 PR 前，请先本地执行：

  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## 📄 授权

MIT — 详见 `LICENSE`。
