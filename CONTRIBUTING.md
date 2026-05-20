# Contributing to Kinemax

## Requirements

- **Rust** stable (1.74+) — install via [rustup](https://rustup.rs/)
- **Python** 3.9 or newer (3.14 recommended; the project pins to 3.14 via `.python-version`)
- **uv** — install via `brew install uv` (macOS) or follow the [uv install guide](https://docs.astral.sh/uv/getting-started/installation/)
- A C linker (Xcode Command Line Tools on macOS, `build-essential` on Linux, MSVC on Windows)

## Quick Start

```bash
git clone https://github.com/anthonysgro/kinemax.git
cd kinemax

# One-shot setup: creates .venv, installs maturin, builds and installs kinemax
uv sync

# Smoke test
uv run python -c "import kinemax; print(kinemax.version())"
```

## Day-to-Day Development

After changes to `src/lib.rs`:

```bash
uv run maturin develop
```

That re-compiles the Rust crate and replaces the installed `.so` (or `.pyd` on Windows) inside the venv. No need to reinstall manually.

For an optimized rebuild matching what CI ships:

```bash
uv run maturin develop --release
```

## Project Structure

```
kinemax/
├── Cargo.toml             # Rust crate metadata + crates.io fields
├── pyproject.toml         # Python package metadata + maturin config
├── src/
│   └── lib.rs             # Rust source — PyO3 bindings live here
├── python/
│   └── kinemax/
│       └── __init__.py    # Pure-Python facade re-exporting the native module
├── .github/workflows/
│   ├── ci.yml             # Per-PR Rust checks + cross-platform build matrix
│   └── release.yml        # Tag-driven publish to PyPI + crates.io + GitHub
├── .python-version        # uv reads this to pick the interpreter
├── LICENSE                # MIT
├── README.md
├── CONTRIBUTING.md        # This file
└── RELEASING.md           # Release pipeline + checklist
```

## Useful Commands

| Command                                              | What it does                                              |
| ---------------------------------------------------- | --------------------------------------------------------- |
| `uv sync`                                            | Create venv, install deps, build & install the extension  |
| `uv run maturin develop`                             | Compile Rust + editable install (debug)                   |
| `uv run maturin develop --release`                   | Compile Rust + editable install (optimized)               |
| `uv run maturin build --release`                     | Build a release wheel for the current platform            |
| `uv run maturin sdist`                               | Build the source distribution                             |
| `uv run python -c "..."`                             | Run a Python command in the project venv                  |
| `cargo check`                                        | Fast type-check of the Rust crate                         |
| `cargo clippy --all-targets -- -D warnings`          | Lint the Rust crate (matches CI)                          |
| `cargo fmt`                                          | Format the Rust crate                                     |
| `cargo fmt --all -- --check`                         | Verify formatting without writing (matches CI)            |

## Pre-PR Checklist

CI runs the same checks, but running them locally first saves a round-trip:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo check --all-targets
uv run maturin develop --release
uv run python -c "import kinemax; print(kinemax.version())"
```

## CI

`.github/workflows/ci.yml` runs on every push to `main` and every PR. It executes:

- Rust checks: `fmt --check`, `clippy -D warnings`, `cargo check`
- Build matrix: Linux + macOS + Windows × Python 3.9, 3.12, 3.14
- Smoke import test on every cell

Concurrency cancels superseded runs on the same branch, so don't worry about stacking pushes.

## Release Pipeline

See [RELEASING.md](RELEASING.md) for the tag-driven publish flow, required secrets, and recovery steps.

## Reporting Issues

Open an issue at <https://github.com/anthonysgro/kinemax/issues>. Helpful info:

- Output of `python --version` and `rustc --version`
- OS and architecture (`uname -ms` on Unix)
- Minimal reproduction
- Stack trace, if any
