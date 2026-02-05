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

詳細的逐步說明請參閱 [.github/copilot-instructions.md](.github/copilot-instructions.md#using-this-template-for-new-projects)。

**設定後的快速驗證**：
```bash
grep -r "rust_template" . --exclude-dir=target --exclude-dir=.git  # 應該只找到少量符合項目
make fmt && cargo build && cargo test --all  # 驗證一切正常
```

## ✨ 重點特色

- 現代 Cargo 結構（`src/lib.rs`、`src/main.rs`、`tests/`）
- 動態版本資訊，包含 git 詮釋資料（標籤、提交雜湊、建置工具）
- clippy + rustfmt 品質把關
- GitHub Actions：測試、品質、打包、Docker 推送、發布草稿、Rust 自動標籤、祕密掃描、語義化 PR、每週依賴更新
- 多階段 Dockerfile，產出精簡執行映像

## 🚀 快速開始

**系統需求：**

- Rust 1.85 或更高版本（使用 Edition 2024）
- Docker（可選）

如尚未安裝 Rust，請使用 `rustup` 進行安裝。

```bash
make fmt            # 格式化 + clippy
make test           # 測試（所有目標）
make test-verbose   # 測試（所有目標與詳細輸出）
make coverage       # 產生 LCOV 覆蓋率報告
make build          # 建置（release 模式）
make build-release  # 發布建置（release 模式）
make run            # 執行（release 模式）
make clean          # 清理建置產物與快取
make package        # 建立 crate 套件（允許 dirty）
make help           # 檢視可用目標
```

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

## 📦 打包發佈

```bash
make package        # 建立 crate 套件（允許 dirty）
# 或直接使用 cargo：
cargo package --locked --allow-dirty
# CARGO_REGISTRY_TOKEN=... cargo publish
```

CI 會在建立 `v*` 標籤時自動打包並上傳 `.crate` 產物。若需自動發佈到 crates.io，請在 `build_package.yml` 開啟發佈步驟並設定密鑰。

## 🧩 跨平台建置

目前模板預設不含本機跨編譯工具。若需在本機使用 cross 或 zig，請依環境安裝與設定。

GitHub Actions `build_release.yml` 會在建立 `v*` 標籤時針對多平台建置釋出二進位，並上傳至 GitHub Release。

目標平台（targets）：

- x86_64-unknown-linux-gnu、x86_64-unknown-linux-musl
- aarch64-unknown-linux-gnu、aarch64-unknown-linux-musl
- x86_64-apple-darwin、aarch64-apple-darwin
- x86_64-pc-windows-msvc、aarch64-pc-windows-msvc

資產命名（assets）：

- `<bin>-v<version>-<target>.tar.gz`（所有平台）
- `<bin>-v<version>-<target>.zip`（Windows 另附）

## 🔁 CI/CD

### 主要工作流程

- 測試（`test.yml`）：建置與測試，生成 LCOV 格式覆蓋率報告並上傳 artifact
- 品質（`code-quality-check.yml`）：rustfmt 檢查 + clippy（拒絕警告）
- 打包（`build_package.yml`）：標籤 `v*` 觸發打包，可選 crates.io 發佈
- 映像（`build_image.yml`）：在 `main/master` 與標籤 `v*` 推送至 GHCR
- 發佈建置（`build_release.yml`）：標籤 `v*` 時建置 Linux 釋出二進位並上傳

### 其他自動化功能

- 自動標籤（`auto_labeler.yml`）：根據分支名稱與檔案變更自動為 PR 添加標籤
- 程式碼掃描（`code_scan.yml`）：多層次安全性掃描（GitLeaks、Trufflehog 祕密掃描、CodeQL 程式碼分析、Trivy 漏洞掃描）
- 發佈草稿（`release_drafter.yml`）：自動生成 release notes
- 語義化 PR（`semantic-pull-request.yml`）：檢查 PR 標題格式
- Dependabot 每週依賴更新

## 🤝 貢獻

- 歡迎 Issue/PR

- PR 標題遵循 Conventional Commits

- 請保持格式化並通過 clippy 檢查

- 每次編輯完畢後，請執行 `cargo build` 來確認編譯是否成功

- 在送出 PR 前，請先本機執行：

  - `cargo fmt --all -- --check`
  - `cargo clippy --all-targets --all-features -- -D warnings`
  - `cargo test`

## 📄 授權

MIT — 詳見 `LICENSE`。
