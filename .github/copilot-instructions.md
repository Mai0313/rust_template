# Copilot Instructions: rust_template

> **TEMPLATE PROJECT NOTICE**: This is a PROJECT TEMPLATE, not a working application.
> When helping users with this codebase, ALWAYS check if they intend to:
>
> 1. **Use this as a template** → Guide them through the "Using This Template" section below
> 2. **Modify the template itself** → Follow normal development workflows
>
> If uncertain, ASK the user first before making changes.

## Project Overview

This is a **multi-language distributed Rust project template**. The core is written in Rust but packaged for distribution via:

- **Cargo** (crates.io) - native Rust users
- **npm** (Node.js) - via CLI wrapper in `cli/nodejs/`
- **PyPI** (Python) - via CLI wrapper in `cli/python/`

The CLI wrappers detect platform/architecture at runtime and spawn pre-compiled native Rust binaries from `binaries/<platform>/`. This enables distributing a single Rust codebase across multiple language ecosystems.

## Using This Template for New Projects

**CRITICAL**: When using this template, you MUST rename all occurrences of `rust_template` to your new project name. This step is required.

### Step 1: Global Find and Replace

Perform a **case-sensitive** find-and-replace across the entire codebase:

- `rust_template` → `your_project_name` (using snake_case)
- `rust-template` → `your-project-name` (using kebab-case, if found)

### Step 2: Rename Python Package Directory

Rename the Python source directory:

```bash
mv cli/python/src/rust_template cli/python/src/your_project_name
```

### Step 3: Update Project Metadata

Update the following information in **all three package manifests**:

#### [Cargo.toml](../Cargo.toml)

```toml
name = "your_project_name"
authors = ["Your Name <your.email@example.com>"]
description = "Your project description"
homepage = "https://github.com/your_username/your_project_name"
repository = "https://github.com/your_username/your_project_name"
```

#### [cli/nodejs/package.json](../cli/nodejs/package.json)

```json
{
  "name": "your_project_name",
  "author": "Your Name <your.email@example.com>",
  "description": "Your project description",
  "homepage": "https://github.com/your_username/your_project_name",
  "repository": {
    "url": "git+https://github.com/your_username/your_project_name.git"
  },
  "bugs": {
    "url": "https://github.com/your_username/your_project_name/issues"
  },
  "bin": {
    "your_project_name": "bin/start.js"
  }
}
```

#### [cli/python/pyproject.toml](../cli/python/pyproject.toml)

```toml
[project]
name = "your_project_name"
authors = [{ name = "Your Name", email = "your.email@example.com" }]
description = "Your project description"

[project.urls]
Homepage = "https://github.com/your_username/your_project_name"
Repository = "https://github.com/your_username/your_project_name"
"Bug Tracker" = "https://github.com/your_username/your_project_name/issues"

[project.scripts]
your_project_name = "your_project_name:main"
```

### Step 4: Update Docker Labels

#### [docker/Dockerfile](../docker/Dockerfile)

```dockerfile
LABEL maintainer="Your Name <your.email@example.com>" \
    org.label-schema.name="your_project_name" \
    org.label-schema.vendor="Your Name"
```

#### [.devcontainer/Dockerfile](../.devcontainer/Dockerfile)

Same as above.

### Step 5: Update GitHub Configuration

#### [.github/CODEOWNERS](../.github/CODEOWNERS)

```
*       @your_github_username
```

#### [.github/workflows/build_release.yml](../.github/workflows/build_release.yml)

If publishing to npm with scoped packages, update lines 199-201:

```yaml
scope: '@your_npm_username'
```

### Step 6: Update README Files

Update all badge URLs and links in:

- [README.md](../README.md)
- [README.zh-CN.md](../README.zh-CN.md)
- [README.zh-TW.md](../README.zh-TW.md)

Replace all instances of:

- `Mai0313/rust_template` → `your_username/your_project_name`
- `rust_template` → `your_project_name`
- Template description → Your project description

### Step 7: Optional Customizations

Consider updating based on your needs:

- **CLI command aliases**: Update `bin` keys in package.json and `scripts` keys in pyproject.toml
- **Keywords**: Update in all three package manifests to reflect your project's focus
- **License**: Update `LICENSE` file and license field in manifests if not using MIT
- **Rust edition**: Update if targeting a different edition (currently 2024)
- **Minimum Rust version**: Update `rust-version` in Cargo.toml if needed (currently 1.85)

### Step 8: Verify Changes

Before committing:

```bash
# 1. Verify all renames completed
grep -r "rust_template" . --exclude-dir=target --exclude-dir=.git

# 2. Build and test
make fmt
cargo build
cargo test --all

# 3. Verify CLI wrappers reference correct binary names
cat cli/nodejs/bin/start.js | grep binary
cat cli/python/src/your_project_name/__init__.py | grep binary
```

### Step 9: Initialize Git Version

Tag your first release to activate build.rs version detection:

```bash
git tag v0.1.0
git push origin v0.1.0
```

**Reminder**: The version in Cargo.toml should remain `0.0.0` - real versions come from git tags via build.rs.

### Common Mistakes to Avoid

1. Remember to rename the Python source directory (`cli/python/src/rust_template/`)
2. Update references in test files (`tests/*.rs`)
3. Update all three README files
4. Replace old author/maintainer info in Dockerfiles
5. Update repository URLs in CI workflows (badges will break otherwise)
6. Verify CLI wrappers work after renaming (test with `node cli/nodejs/bin/start.js`)

---

## Critical Architecture Patterns

### 1. Version Information via build.rs

The project uses a sophisticated **build-time version injection system** in [build.rs](../build.rs):

- Captures git metadata: latest tag, commit count since tag, short commit hash, dirty state
- Embeds Rust and Cargo versions used for the build
- Exposes via environment variables: `BUILD_VERSION`, `BUILD_RUST_VERSION`, `BUILD_CARGO_VERSION`
- Access in code: `rust_template::version()`, `rust_template::rust_version()`, `rust_template::cargo_version()`

**Key detail**: Version format is `<tag>-<commit-count>-g<hash>[-dirty]`, e.g., `v0.1.25-2-gf4ae332-dirty`

### 2. Multi-Platform Binary Distribution

CLI wrappers ([cli/nodejs/bin/start.js](../cli/nodejs/bin/start.js), [cli/python/src/rust_template/__init__.py](../cli/python/src/rust_template/__init__.py)) follow this pattern:

1. Detect current platform and architecture (`process.platform`/`platform.system()`)
2. Map to binary subdirectory: `linux-x64-gnu`, `macos-arm64`, `windows-x64`, etc.
3. Spawn the native binary from `binaries/<platform>/rust_template[.exe]`
4. Pass through all arguments and exit codes

**When modifying**: Update both Node.js and Python wrappers in parallel. Platform mappings must stay consistent.

### 3. Library vs Binary Organization

- **[src/lib.rs](../src/lib.rs)**: Public API functions plus unit tests inside a `#[cfg(test)] mod tests` block at the bottom of the file.
- **[src/main.rs](../src/main.rs)**: Binary entry point — a thin wrapper over the library. Deliberately has no `tests` module; test business logic in `lib.rs` instead.
- **[tests/](../tests/)**: Integration tests that use only the public API (`rust_template::*`). Each file becomes its own crate.

**Convention**: All business logic lives in `lib.rs`. The binary is a thin wrapper that calls library functions.

## Developer Workflows

### Pre-commit Quality Checks (Run Locally Before PR)

```bash
pre-commit run -a     # Run ALL pre-commit hooks (must pass before committing)
make fmt              # Format + clippy (auto-fix)
cargo fmt --all -- --check  # Verify formatting (CI enforces this)
cargo clippy --all-targets --all-features -- -D warnings  # CI fails on warnings
cargo test --all      # All tests must pass
```

CI enforces formatting and clippy warnings-as-errors. **Always run `make fmt` before committing.**

**After every code change, always run `pre-commit run -a` to ensure all hooks pass before committing.**

### Build and Test Commands

```bash
make build           # Debug build
make release         # Release build (optimized)
make test            # Run all tests (unit + integration)
make test-verbose    # Tests with full output
make coverage        # Generate LCOV coverage (requires cargo-llvm-cov)
make run             # Run the release binary
make package         # Build .crate file for publishing
make clean           # Clean all build artifacts and caches
```

**Key detail**: `cargo build` alone is sufficient to verify compilation. CI uses `cargo build` + `cargo test`.

### Docker Workflow

The [Dockerfile](../docker/Dockerfile) is a multi-stage build:

1. **builder stage**: Alpine + Rust 1.89, compiles release binary
2. **prod stage**: Alpine minimal, copies only the binary

```bash
docker build -f docker/Dockerfile --target prod -t rust_template:latest .
docker run --rm rust_template:latest  # Runs the binary as entrypoint
```

## CI/CD Pipeline Structure

Located in [.github/workflows/](workflows/):

| Workflow                    | Trigger               | Purpose                                                          |
| --------------------------- | --------------------- | ---------------------------------------------------------------- |
| `test.yml`                  | PR/push               | Build, test, generate LCOV coverage                              |
| `code-quality-check.yml`    | PR/push               | Enforce rustfmt + clippy (deny warnings)                         |
| `build_release.yml`         | tag `v*`              | Cross-compile binaries for 8 platforms, upload to GitHub Release |
| `build_image.yml`           | main/master, tag `v*` | Build and push Docker image to GHCR                              |
| `auto_labeler.yml`          | PR                    | Auto-label PRs based on files changed (Rust-aware)               |
| `semantic-pull-request.yml` | PR                    | Enforce Conventional Commits titles                              |
| `code_scan.yml`             | schedule              | Security: GitLeaks, Trufflehog, CodeQL, Trivy                    |
| `release_drafter.yml`       | release               | Auto-generate release notes                                      |
| `pre-commit-updater.yml`    | daily                 | Update pre-commit hooks                                          |
| `auto_review_merge.yml`     | PR                    | Dependency review + auto-merge Dependabot PRs                    |

**Platform targets** for release builds (8 total):

- Linux: x86_64/aarch64 (gnu and musl)
- macOS: x86_64/aarch64 (darwin)
- Windows: x86_64/aarch64 (msvc)

## Project-Specific Conventions

### 1. Cargo Configuration

[Cargo.toml](../Cargo.toml):

- Rust **Edition 2024**, requires **1.85+**
- Release profile: `lto = "thin"`, `codegen-units = 1`, `strip = "symbols"` (optimized for size/speed)
- Version is `0.0.0` in Cargo.toml (overridden by git tags via build.rs)

### 2. Testing Standards

This project follows Rust's idiomatic two-tier test layout:

- **Unit tests** live inside each source file under `src/`, wrapped in `#[cfg(test)] mod tests { ... }`. They can touch private items. Keep them focused on a single function or small unit, and cover edge cases (zero, negatives, large values). Within the `tests` module, group tests into one sub-module per tested function (e.g., `tests::add`, `tests::multiply`) so the test path encodes what is being tested. See [src/lib.rs](../src/lib.rs).
- **Integration tests** live in the top-level [tests/](../tests/) directory. Each file is compiled as a separate crate and may **only** use the public API (`rust_template::*`). Name each file after the topic it covers and split by concern — in this template, [tests/arithmetic.rs](../tests/arithmetic.rs) handles cross-function composition and invariants, and [tests/version.rs](../tests/version.rs) handles build-time version metadata. Avoid a generic catch-all file like `basic.rs`, and do not duplicate unit-test cases.
- **Shared integration-test helpers** belong in `tests/common/mod.rs` (the subdirectory form), not `tests/common.rs`, so Cargo does not treat them as a separate integration crate. See [the Rust Book, ch. 11.3](https://doc.rust-lang.org/book/ch11-03-test-organization.html).
- **Do not** add a `tests` module to `src/main.rs` — the binary is a thin wrapper; test the underlying library functions in `src/lib.rs` instead.

### 3. GitHub Actions Formatting Conventions

When editing or creating GitHub Actions workflow files, follow these rules:

- **Do not** include `container` fields or `Setup MTK Certification` steps.
- **Job attribute order**: `name`, `needs`, `runs-on`, `if` (followed by other attributes such as `strategy`, `steps`, etc.)
- **Step attribute order**: `name`, `id`, `continue-on-error`, `if`, `uses`, `with`, `env`, `shell`, `run`
- **Avoid redundant environment variables**: Do not define env vars (e.g., `PR_URL: ${{ github.event.pull_request.html_url }}`) that are only used once in a `run` command. Use the expression directly in the command instead.

### 4. Multi-Language Package Metadata

All three package manifests ([Cargo.toml](../Cargo.toml), [cli/nodejs/package.json](../cli/nodejs/package.json), [cli/python/pyproject.toml](../cli/python/pyproject.toml)) share:

- Same description, keywords, author, license, homepage, repository URL
- **When updating metadata**: Sync across all three files to maintain consistency

## Git Conventions

### Commit Messages

- **All commit messages must be written in English** — no other languages are accepted.

- Commit messages must follow the **Conventional Commits** specification:

    ```
    <type>[optional scope]: <description>

    [optional body]

    [optional footer(s)]
    ```

- Common types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `perf`, `revert`

- Examples:

    - `feat(cli): add --verbose flag to output detailed logs`
    - `fix(auth): handle token expiry edge case`
    - `docs: update README with new installation steps`
    - `chore: bump pre-commit hook versions`

### Pull Request Titles

- **All PR titles must be written in English** — no other languages are accepted.
- PR titles must also follow the **Conventional Commits** format.
- The PR title becomes the squash-merge commit message, so it must be descriptive and accurate.

## Common Pitfalls

1. **Version in Cargo.toml is intentionally `0.0.0`**: Real version comes from git tags via build.rs. Keep it as `0.0.0`.
2. **Clippy warnings become errors in CI**: Fix all warnings locally with `make fmt` before pushing.
3. **CLI wrappers must stay in sync**: When adding new platforms, update both Node.js and Python wrappers.
4. **build.rs depends on git state**: Fails gracefully if not in a git repo, but version info won't be accurate.
5. **Release profile strips symbols**: For debugging release builds, temporarily remove `strip = "symbols"` from Cargo.toml.

## Critical Usage Guidelines

- **After every code change, always run `pre-commit run -a` before committing to ensure all hooks pass.**
- See the [Git Conventions](#git-conventions) section for commit message and PR title requirements.

## Key Files Reference

- [src/lib.rs](../src/lib.rs) - Public API, version functions, unit tests
- [src/main.rs](../src/main.rs) - Binary entry point
- [build.rs](../build.rs) - Build-time version injection from git
- [Makefile](../Makefile) - Developer workflow commands
- [docker/Dockerfile](../docker/Dockerfile) - Multi-stage Docker build
- [cli/nodejs/bin/start.js](../cli/nodejs/bin/start.js) - Node.js CLI wrapper
- [cli/python/src/rust_template/__init__.py](../cli/python/src/rust_template/__init__.py) - Python CLI wrapper
- [tests/arithmetic.rs](../tests/arithmetic.rs) - Integration tests for arithmetic composition and invariants
- [tests/version.rs](../tests/version.rs) - Integration tests for build-time version metadata
