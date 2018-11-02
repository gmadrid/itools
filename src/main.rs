extern crate itools;

use std::error::Error;

use itools::{expand_file_list, Config, HashMaster, ItoolsError, Progrs, Result};

// - Different hashers
//   - Mean
//   - Diff
//   - Perceptual
//   - MD5
// - Opener
// - Persistence
//   - save
//   - load
// - Progress
//   - quiet mode

fn run() -> Result<()> {
    let config = Config::new()?;
    let (files, _missing) = expand_file_list(config.files)?;

    // TODO: report the missing files.
    let p = Progrs::new(files.len() as u64);
    // TODO: move this into run, maybe?
    HashMaster::new(files).run(&p)?;

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
