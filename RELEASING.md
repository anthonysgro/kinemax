# Release Cheatsheet

## Unified Release

Kinemax ships both registries from a single tag push. One `v*` tag produces:

- **PyPI wheels** — Linux (x86_64, aarch64), macOS (x86_64, aarch64), Windows (x64, aarch64)
- **PyPI sdist** — for source-only installs and platforms not in the matrix
- **crates.io publish** — Rust source distribution
- **GitHub Release** — every wheel + sdist attached, auto-generated notes, SLSA attestations

PyPI uploads use [Trusted Publishing](https://docs.pypi.org/trusted-publishers/), so no PyPI token lives in GitHub Secrets. crates.io still needs `CARGO_REGISTRY_TOKEN` configured under repo Settings → Secrets and variables → Actions.

## Releasing

```bash
# 1. Bump the version in BOTH manifests (must match exactly)
#    - Cargo.toml      → [package] version = "0.0.2"
#    - pyproject.toml  → [project]  version = "0.0.2"

# 2. Sanity-check locally
cargo fmt --check
cargo clippy --all-targets -- -D warnings
uv run maturin develop --release
uv run python -c "import kinemax; print(kinemax.version())"

# 3. Commit, tag, push
git add Cargo.toml pyproject.toml
git commit -m "Release v0.0.2"
git tag v0.0.2
git push origin main --tags
```

The release workflow then:

1. **`verify-version`** — fails fast if the tag, `Cargo.toml`, and `pyproject.toml` disagree
2. Builds wheels in parallel across Linux, macOS, Windows
3. Builds the sdist
4. Publishes to PyPI via Trusted Publishing (uses the `pypi` GitHub Environment)
5. Publishes the crate to crates.io
6. Creates a GitHub Release with all wheels + the sdist attached and SLSA artifact attestations

## Required Secrets and Environments

| Where           | Name                    | Purpose                                                             |
| --------------- | ----------------------- | ------------------------------------------------------------------- |
| GitHub Secret   | `CARGO_REGISTRY_TOKEN`  | crates.io publish-update token scoped to `kinemax`                  |
| GitHub Env      | `pypi`                  | Gates the PyPI publish job; wired to PyPI's Trusted Publisher       |
| PyPI Publisher  | `anthonysgro/kinemax`   | Workflow `release.yml`, environment `pypi`                          |

## Local Build (Testing)

```bash
# One-shot setup if you don't have a venv yet
uv sync

# Editable install for development
uv run maturin develop

# Release-mode build of a wheel for the current platform
uv run maturin build --release

# Source distribution (used as the fallback on PyPI)
uv run maturin sdist

# Wheels and sdist land in target/wheels/
ls target/wheels/
```

## Useful Git Commands

```bash
git tag                       # list all tags
git tag -d v0.0.2             # delete a local tag
git push origin :v0.0.2       # delete a remote tag (pulls release in flight)
git log --oneline --decorate  # see commits with tags
```

## Recovering From a Bad Release

- **PyPI**: versions cannot be re-uploaded. Yank the bad version (`pip-only` deprecation) and ship a fresh patch:
  ```bash
  # via web UI: pypi.org → manage project → release → Yank
  ```
- **crates.io**: same story, no overwrite. Yank with:
  ```bash
  cargo yank --version 0.0.2
  ```
  Yank only hides the version from new dependents; existing `Cargo.lock` files keep working.
- **GitHub**: delete the release, delete the tag locally and on origin, fix, retag, push.
