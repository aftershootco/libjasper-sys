mod build;
mod clone;
mod error;
use std::path::PathBuf;

use error::Result;
pub fn main() -> Result<()> {
    __check();
    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let jasper_url = std::env::var("JASPER_URL")
        .unwrap_or_else(|_| "https://github.com/jasper-software/jasper".into());
    #[cfg(feature = "build")]
    clone::clone(jasper_url, out_dir.join("jasper"))?;
    #[cfg(feature = "build")]
    build::build(&out_dir, out_dir.join("jasper"))?;

    Ok(())
}

pub fn __check() {
    #[cfg(all(feature = "build_native", feature = "build_cc"))]
    compile_error!("Only one of [build_native, build_cc] features can be enabled");
}
