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

详细的分步说明请参阅 [CLAUDE.md](CLAUDE.md)。

**设置后的快速验证**：

```bash
grep -r "rust_template" . --exclude-dir=target --exclude-dir=.git  # 应该只找到少量匹配
make fmt && cargo build && cargo test --all  # 验证一切正常
```

## ✨ 特色

- 现代 Cargo 结构：unit tests 放在 `src/` 内，integration tests 放在 `tests/`
- 动态版本信息，包含 git 元数据（标签、提交哈希、构建工具）
- clippy + rustfmt 质量保障
- GitHub Actions：测试、质量、打包、Docker 推送、发布草稿、Rust 自动加标签、秘密扫描、语义化 PR、每周依赖更新
- 多阶段 Dockerfile，产出精简运行镜像

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

## 🛠️ 开发

开发环境设置、常用命令、测试、代码规范、CI 与发布流程都在 [CONTRIBUTING.md](./.github/CONTRIBUTING.md)。

## 📄 授权

MIT — 详见 `LICENSE`。
