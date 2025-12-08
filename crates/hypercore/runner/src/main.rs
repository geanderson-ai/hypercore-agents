use anyhow::Result;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::env;
use std::fs;

fn main() -> Result<()> {
    // initialize python interpreter
    pyo3::prepare_freethreaded_python();
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: hypercore-runner path/to/script.py");
        std::process::exit(1);
    }
    let path = &args[1];
    
    println!("HyperCore Runner: Executing {}", path);

    // Read script content
    // Note: run_path can handle reading, but having explicit read allows pre-processing if needed.
    // We will stick to run_path with filename for correct __file__ attribute.
    
    Python::with_gil(|py| -> PyResult<()> {
        // Ensure site-packages from venv are available if running from outside.
        // Usually handled by environment variables (VIRTUAL_ENV), but good to know.
        
        let runpy = py.import("runpy")?;
        
        // Fix sys.argv so the script sees its own arguments
        let sys = py.import("sys")?;
        sys.getattr("argv")?.call_method1("clear", ())?;
        sys.getattr("argv")?.call_method1("append", (path,))?;
        // Append other args if any
        for arg in args.iter().skip(2) {
             sys.getattr("argv")?.call_method1("append", (arg,))?;
        }

        // Execute
        let result = runpy.call_method1("run_path", (path,));
        
        if let Err(e) = result {
             eprintln!("Error executing script:");
             e.print(py);
             return Err(e);
        }
        Ok(())
    })?;

    Ok(())
}
