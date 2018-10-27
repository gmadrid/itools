extern crate itools;

use std::error::Error;

use itools::{expand_file_list, Config, HashMaster, ItoolsError, Result};

fn run() -> Result<()> {
    let config = Config::new()?;
    let (files, _missing) = expand_file_list(config.files)?;

    // TODO: report the missing files.

    HashMaster::new(files).run()?;

    Ok(())
}

fn main() {
    // TODO: deal with Clap errors by printing usage.
    match run() {
        Ok(_) => (),
        Err(ItoolsError::Clap(err)) => println!("{}", err.description()),
        Err(e) => println!("Error: {:?}", e),
    }
}
