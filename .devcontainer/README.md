# Dev Container for Rust Project

This directory contains configuration files for developing this project in a fully reproducible environment using [VS Code Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers).

## What's Included?

- **Dockerfile**: Builds an image based on Rust 1-1-bookworm (Debian), with additional tools:
    - **uv**: Fast Python package installer (for CLI wrappers in `cli/python/`)
    - **zsh + oh-my-zsh + powerlevel10k**: Modern terminal experience with auto-suggestions and syntax highlighting
    - **Build tools**: gcc, clang, lld, cmake for native Rust dependencies
    - **Nerd Fonts**: MesloLGS NF for proper terminal rendering
- **devcontainer.json**: VS Code configuration for the container, including:
    - **Recommended extensions**:
        - Rust: `rust-analyzer`, `even-better-toml`
        - Python: `python`, `pylance`, `debugpy` (for CLI wrappers)
        - Jupyter: Full Jupyter suite (for notebooks)
        - Docker: `vscode-docker`, `vscode-containers`
        - Database: `dbclient-jdbc`, `vscode-redis-client`
        - Git: `gitlens`
        - AI: `github.copilot-chat`, `huggingface-vscode-chat`
        - Quality of life: `indent-rainbow`, `reload`, `default-keys-windows`
    - **Custom mounts**: Local `.gitconfig`, `.ssh`, and `.p10k.zsh` for seamless integration
    - **Auto-update**: Runs `rustup update && rustup component add clippy rustfmt && cargo fetch` on container start

## Usage

1. **Open this folder in VS Code** (with the [Dev Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) installed).
2. **Reopen in Container** when prompted, or use the command palette: `Dev Containers: Reopen in Container`.
3. The environment will be built automatically:
    - Rust toolchain updated
    - Clippy and rustfmt installed
    - Cargo dependencies pre-fetched

## Customization

- **Change Rust toolchain**: The base image uses Debian Bookworm with Rust 1.x. To pin a specific version, modify the Dockerfile `FROM` line.
- **Add system packages**: Edit the Dockerfile to install additional build dependencies via `apt-get`.
- **Add VS Code extensions**: Update the `extensions` list in `devcontainer.json`.
- **Mount more files**: Add to the `mounts` array in `devcontainer.json` (e.g., AWS credentials, custom config files).

## Useful Commands

- **Rebuild container**: Use `Dev Containers: Rebuild Container` from the command palette after changing Dockerfile or devcontainer.json.
- **Update Rust toolchain**: Run `rustup update` in the container terminal.
- **Build the project**: Use `make build` or `cargo build` as usual.

## Troubleshooting

- **SSH/Git issues**: Ensure your local `.ssh` and `.gitconfig` files exist and are correctly mounted (read-only by default).
- **Rust analyzer slow**: First build may take time as it indexes the project and downloads dependencies.
- **Permission issues**: The container runs as the `vscode` user. Check file ownership if you encounter write errors.
- For more info, see the [VS Code Dev Containers documentation](https://code.visualstudio.com/docs/devcontainers/containers).
