# HyperCore Runtime MVP: Build & Run

All files are located in `crates/hypercore`.

## Prerequisites
- Rust (cargo)
- Python 3.10+
- `pip install maturin`

## Step-by-Step Setup

1. **Create and Activate Virtual Environment** (Recommended)
   ```bash
   cd crates/hypercore
   python -m venv .venv
   # Windows
   .venv\Scripts\activate
   # Linux/Mac
   # source .venv/bin/activate
   pip install maturin
   ```

2. **Build & Install Native Extension**
   This compiles the Rust code in `native/` and installs it into your current environment.
   ```bash
   cd native
   maturin develop --release
   cd ..
   ```

3. **Install Python Shim** (Optional but recommended for IDE support)
   ```bash
   cd python-shim
   pip install -e .
   cd ..
   ```

4. **Build the Runner Binary**
   ```bash
   cd runner
   cargo build --release
   cd ..
   ```

5. **Run the Example Script**
   Use the compiled runner to execute the script. The runner will load the `hypercore_native` module from your environment (or side-by-side if configured).
   ```bash
   # Make sure your VENV is active so the runner finds the installed extension
   ./runner/target/release/hypercore-runner script.py
   ```

## Troubleshooting
- If `hypercore_native` not found: Ensure you ran `maturin develop` inside the *active* virtual environment you are using to run the script.
