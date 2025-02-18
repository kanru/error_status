use std::io::{self, ErrorKind};

use anyhow::Result;
use error_status::ResultExt;

fn find_file() -> Result<(), io::Error> {
    Err(ErrorKind::NotFound.into())
}

fn main() -> Result<()> {
    find_file()
        .not_found("Failed to read file")
        .internal_error("Config file is not available")?;
    Ok(())
}
