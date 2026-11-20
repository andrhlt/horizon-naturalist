# Naturalist for Horizon

This branch is an **unfinished development port** of Naturalist 2.0.7 for Minecraft 26.3 and Horizon `0a875da91dbb0fdcb7ba8c42c11341aa229af51a`. It builds as a native `.hmod`, but most species gameplay and rendering remain to be migrated. It is not a release.

## Install, build, play

In the Horizon launcher, choose your instance and install the development branch from its GitHub link:

`https://github.com/andrhlt/horizon-naturalist/tree/codex/naturalist-current-integration`

Select **Build**, then **Play**. The launcher may ask to download the exact original Naturalist jar from Modrinth; approve that prompt or supply it in the instance's `jars/<sha512>.jar` cache. The manifest pins its SHA-512, and the original assets and data are not redistributed here. See [LICENSE-Naturalist](LICENSE-Naturalist).

The root `horizon-mod.toml`, `Cargo.toml`, `src/current/`, and `data/` are the development project. A source checkout can be built with `horizon-launcher native-build .`. For development packaging, `python3 tools/package_current.py /path/to/naturalist-2.0.7-neoforge-26.3.jar /tmp/naturalist-dev.hmod` writes a deterministic package; `python3 tools/refresh_current_data.py <jar>` checks the generated data, and adding `--write` regenerates it. Neither Python command is required to install the project in the launcher. See [migration notes](migration/README.md) for tested behavior and known gaps.

The historical 0.7.4-preview.45 port, its launcher helpers, and its original documentation remain in the repository's `main` branch and Git history. They are not the current `.hmod` implementation.
