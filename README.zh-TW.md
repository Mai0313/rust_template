<div align="center" markdown="1">

# Rust 專案模板

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

🚀 幫助 Rust 開發者「快速建立新專案」的模板。內建 Cargo 佈局、Docker 與完整 CI/CD 流程。

點擊 [使用此模板](https://github.com/Mai0313/rust_template/generate) 後即可開始。

其他語言: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## 🎯 使用此模板

**重要提醒**：這是一個模板儲存庫。在將其用於您的專案之前，您必須：

1. **重新命名所有出現的** `rust_template` 為您的專案名稱（整個程式碼庫）
2. **更新詮釋資料**：修改 `Cargo.toml`、`cli/nodejs/package.json` 和 `cli/python/pyproject.toml`
3. **更新作者資訊**：修改所有套件清單和 Dockerfile 中的作者資訊
4. **更新儲存庫 URL**：修改 README 徽章、套件清單和 GitHub workflows 中的連結
5. **重新命名 Python 套件目錄**：將 `cli/python/src/rust_template` 改為您的專案名稱

詳細的逐步說明請參閱 [CLAUDE.md](CLAUDE.md)。

**設定後的快速驗證**：

```bash
grep -r "rust_template" . --exclude-dir=target --exclude-dir=.git  # 應該只找到少量符合項目
make fmt && cargo build && cargo test --all  # 驗證一切正常
```

## ✨ 重點特色

- 現代 Cargo 結構：unit tests 放在 `src/` 內，integration tests 放在 `tests/`
- 動態版本資訊，包含 git 詮釋資料（標籤、提交雜湊、建置工具）
- clippy + rustfmt 品質把關
- GitHub Actions：測試、品質、打包、Docker 推送、發布草稿、Rust 自動標籤、祕密掃描、語義化 PR、每日依賴更新
- 多階段 Dockerfile，產出精簡執行映像

## 📌 版本資訊

執行檔會自動顯示動態版本資訊，包含：

- Git 標籤版本（若無標籤則使用 `Cargo.toml` 版本）
- 自上次標籤以來的提交數量
- 簡短提交雜湊值
- 工作目錄是否有未提交的更改（dirty 標記）
- 建置時使用的 Rust 與 Cargo 版本

輸出範例：

```
rust_template v0.1.25-2-gf4ae332-dirty
Built with Rust 1.90.0 and Cargo 1.90.0
```

這些版本資訊會在建置時透過 `build.rs` 自動嵌入，並根據你的 git 狀態動態更新。

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t ghcr.io/<owner>/<repo>:latest .
docker run --rm ghcr.io/<owner>/<repo>:latest
```

或使用實際的二進位名稱：

```bash
docker build -f docker/Dockerfile --target prod -t rust_template:latest .
docker run --rm rust_template:latest
```

## 🛠️ 開發

開發環境設定、常用指令、測試、程式碼規範、CI 與發佈流程都在 [CONTRIBUTING.md](./.github/CONTRIBUTING.md)。

## 📄 授權

MIT — 詳見 `LICENSE`。
