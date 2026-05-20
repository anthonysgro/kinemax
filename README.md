# Kinemax

**Summary:** Kinemax is a high-performance spatial-temporal computing engine designed for analyzing high-frequency coordinate and kinematics tracking data. *"A local-first execution runtime for fixed-cardinality spatiotemporal entity systems."*

## The Vision

Whether tracking player telemetry on a sports field, monitoring mobile entities within fixed boundaries, or processing continuous $X,Y,Z$ positional vectors over time, Kinemax provides a lightweight, local-first framework optimized for rapid geometric and physical calculations. Built with a low-dependency Rust core and paired with an idiomatic Python API, it bridges the gap between systems-level execution speed and rapid data-science workflows.

> **Note:** This package is currently in active pre-alpha development. The namespace is reserved as the underlying architecture is being established.

## Quick Start (Development)

Requires [uv](https://docs.astral.sh/uv/) and a working Rust toolchain.

```bash
# One-shot setup: creates .venv, installs maturin, builds and installs kinemax
uv sync

# Verify the install
uv run python -c "import kinemax; print(kinemax.version())"
```

After editing `src/lib.rs`, rebuild with:

```bash
uv run maturin develop
```

## License

MIT
