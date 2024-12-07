use std::{fs, path::Path};

use anyhow::{bail, Result};

pub fn read_file(input_path: &Path) -> Result<fs::File> {
    if !input_path.exists() || !input_path.is_file() {
        bail!("Invalid file '{}'!", input_path.to_string_lossy());
    }

    return Ok(fs::File::open(input_path)?);
}
