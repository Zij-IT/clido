use anyhow::{bail, Result};
use std::path::PathBuf;

pub fn clido_dir() -> Result<PathBuf> {
    let data_dir = match std::env::var_os("_CLIDO_DIR") {
        Some(os_str) => PathBuf::from(os_str),
        None => match dirs_next::data_local_dir() {
            Some(mut dir) => {
                dir.push("clido");
                dir
            }
            None => bail!("Could not find data directory. Please set _CLIDO_DIR."),
        },
    };

    Ok(data_dir)
}
