use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))?.write_all(include_bytes!("memory.x"))?;

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    Ok(())
}