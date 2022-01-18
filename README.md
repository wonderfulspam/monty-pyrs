# PoC: Integrating Rust in Python

A toy example showing how to run Rust code in Python for speed and
progress.

## Requirements

* Python 3.6+
* Rust 1.44+
* Cargo (bundled with Rust)

## Setup

```bash
python -m venv .venv
source .venv/bin/activate # or source .venv/bin/activate.fish
pip install maturin
maturin develop --release # Pass --release flag to Cargo for speedups
python run.py # Runs 1_000_000_000 Monty Hall simulations
```

## Credits

[PyO3](https://github.com/PyO3/pyo3) which provides macros for exposing
Rust code to Python and [Maturin](https://maturin.rs) which handles the
scaffolding.
