# Initializing a New Project from This Rust Template

This repository is a Rust CLI template with distribution paths for crates.io, npm, PyPI, and GitHub Releases. It is not the product itself. First establish whether the user wants to initialize a new project or maintain the template. If the intent is unclear, ask before changing names, package identities, or release settings.

Initialization preserves the existing capabilities while replacing the template identity with the new project's identity. Complete the naming, metadata, documentation, and CI migration before adding product features. Do not treat initial setup as an opportunity to remove template capabilities.

## Inventory Before Editing

1. Read `README.md`, `README.zh-TW.md`, `README.zh-CN.md`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `Makefile`, `build.rs`, `src/`, `tests/`, `cli/nodejs/`, `cli/python/`, `docker/`, `.devcontainer/`, and `.github/`.
2. Check `git status --short`, the current branch, existing remotes, the intended GitHub owner and repository name. Do not overwrite initialization work already made by the user.
3. Confirm the product name, Rust crate name, CLI command, GitHub owner and repository, author, license, description, supported platforms, registries to publish to, Docker image name, and first-version policy. These values may be related but must not be assumed to be identical.
4. Before changing files, locate every real occurrence of `rust_template`, `Mai0313`, `mai0313.com`, `rt`, and the old repository URLs. Exclude `.git/` and `target/` from searches. Treat `Cargo.lock` as a Cargo-generated artifact, not a hand-edited source of truth.

## Name and Metadata Mapping

Choose a deliberate value for each identity instead of applying a blind global replacement.

1. Map the Cargo package name, Rust library crate identifier, and CLI binary name separately. A registry package may use hyphens or underscores, while Rust source refers to a valid identifier and normally normalizes package-name hyphens to underscores unless `[lib].name` overrides it. The `[[bin]].name` value is the installed executable. Keep these choices aligned with `src/main.rs`, integration tests, `Cargo.lock`, the Docker release binary, and the binary names expected by the Node and Python wrappers.
2. The CLI command is a public interface. Both the npm and Python wrappers currently expose `rt` and `rust_template`. Replace these with the new product command and only the aliases that the project intentionally supports. Do not accidentally retain a template alias as a public command.
3. An npm package name may use kebab case or a scope. A Python distribution name may differ from its import package, but `cli/python/src/<package>/` and the `[project.scripts]` import target must be valid Python identifiers. After renaming the directory, update the script target and the binary name referenced inside the package.
4. Update description, authors, license, homepage, repository, issue URL, and keywords together in `Cargo.toml`, `cli/nodejs/package.json`, and `cli/python/pyproject.toml`. The three manifests must describe the same product while following each registry's format.
5. Update labels in `docker/Dockerfile`. Update Docker build commands, copy paths, and `ENTRYPOINT` so they point to the new release binary. `.devcontainer/` has no Dockerfile and carries no project identity, so it normally needs no rename.
6. Regenerate and inspect `Cargo.lock` through `cargo check` or `cargo build`. Do not edit the lockfile directly.

## Template References That Need Deliberate Treatment

The template identity appears outside the package manifests.

1. `src/main.rs`, `src/lib.rs`, `tests/arithmetic.rs`, and `tests/version.rs` contain crate paths, displayed text, and example arithmetic behavior. Replace the examples with the new product's smallest useful behavior while preserving a library-first design and a thin binary layer. Keep private-behavior unit tests beside source code and public-behavior integration tests in `tests/`.
2. `build.rs` embeds the latest git tag, commit count, short hash, dirty state, Rust version, and Cargo version into the binary. Preserve this version source unless the project explicitly adopts another release version model. The template's `Cargo.toml` version is `0.0.0`; the release workflow writes the tag version before publishing. Do not confuse the development version with the registry release version.
3. `cli/nodejs/bin/start.js` and `cli/python/src/<package>/__init__.py` find native binaries under `binaries/<platform>/` based on OS and architecture. The current Linux runtime mapping selects the GNU asset, while macOS and Windows select their matching assets. When changing a binary name, package directory, or supported platforms, verify both wrappers, release-asset staging, and the resulting package contents together.
4. `.github/workflows/build_release.yml` builds release assets for six Rust targets from `v*` tags, packages the crate, and publishes npm and PyPI wrappers. Recheck tag matching, Cargo version injection, npm scope, PyPI project name, binary extraction paths, registry-token secrets, and post-publication verification. A manifest-only change is not proof that release publishing works.
5. Check `.github/CODEOWNERS`, `.github/labeler.yml`, issue templates, release-drafter configuration, Dependabot configuration, the Docker-image workflow, and all GitHub owner or repository URLs. Do not leave the old owner in a badge, image tag, or automation permission setting.

## README and Documentation Rules

The three README files are language variants of the same product documentation. Update the product name, introduction, installation, usage examples, Docker image, package-registry links, repository links, and the link to this file.

Existing badges at the beginning of every README must never be deleted. They may only be modified for the new project, such as package names, GitHub owner or repository, workflow filenames, and license links, or supplemented with additional badges. Keep every existing badge even when its distribution channel is not enabled yet. Point it at the correct intended destination and state the channel's status in the documentation instead of removing the badge.

Synchronize executable commands and version explanations across all three README files. If an installation route is not available yet, state that explicitly rather than leaving an example that invokes the template binary.

## GitHub Actions and Release Configuration

During initialization, keep every existing workflow and modify it for the new repository. This includes `test.yml`, `code-quality-check.yml`, `code_scan.yml`, `build_release.yml`, `build_image.yml`, `auto_labeler.yml`, `auto_review_merge.yml`, `release_drafter.yml`, `semantic-pull-request.yml`, and `pre-commit-updater.yml`.

For each workflow, confirm triggers, branch names, permissions, required secrets, package scope, container registry, release-asset names, and support-platform assumptions. For example, if registry credentials are not available, retain build verification and protect publishing steps with an explicit condition. Do not remove a workflow just to make CI pass.

Only GitHub Actions may be reduced later. Remove a workflow, job, or release path only after there is a verified replacement or a demonstrated lack of need, and after the user explicitly confirms the reduction. Before doing so, update any affected README badges, documentation, manifests, wrapper aliases, release instructions, and required checks. This later reduction rule never permits deleting existing README badges.

## Recommended Implementation Order

1. Create a name and metadata mapping, then update the manifests, package directory, Rust crate path, wrappers, Docker labels, and CODEOWNERS.
2. Replace the example API in `src/lib.rs` with the initial domain API. Keep `src/main.rs` as a thin caller of the library and add unit and integration tests for the new behavior.
3. Update all three READMEs and GitHub workflow references to URLs, badges, image names, release names, package names, and secret usage.
4. Check `.pre-commit-config.yaml`, `rust-toolchain.toml`, `Makefile`, the devcontainer, and Docker base images against the new project's needs. During initialization, modify rather than remove existing quality and security checks.
5. Keep initialization changes small and traceable. If publication is in scope, validate the full binary-delivery path with a prerelease or test registry before creating the first production `v*` tag.

## Completion Criteria

Run the checks appropriate to each substantial change. Before considering initialization complete, run:

```bash
pre-commit run -a
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo build --locked
cargo package --locked --allow-dirty
rg -n -i --hidden -g '!.git/**' -g '!target/**' -g '!CLAUDE.md' 'rust_template|Mai0313|mai0313\.com|copilot-instructions' .
```

The final search should return only intentional historical or template references. Account for every retained result. If npm or PyPI publication remains in scope, build each wrapper package, verify that `binaries/` contains the expected assets, and on available Linux, macOS, or Windows environments verify that the target CLI starts, forwards arguments, and preserves its exit code.

Before creating a release, use a clean checkout to verify that `build.rs` provides sensible version information with a tag, without a tag, and outside a git repository. Use a draft release or test tag to confirm that `build_release.yml` produces the expected crate, release assets, npm publishing conditions, and PyPI publishing conditions.

## Working Principles

Do not hide initialization problems by deleting tests, linters, badges, workflows, or package wrappers. Identify the mismatch between a template assumption and the new product's requirements, change the configuration or implementation, then prove that the pieces work together with the relevant verification.

Commit messages and GitHub pull-request titles must use English Conventional Commits. Unless the user explicitly requests otherwise, keep initialization work in a draft pull request until all checks pass, then ask whether it should be marked ready for review.
