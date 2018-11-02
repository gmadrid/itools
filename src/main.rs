extern crate itools;

use std::error::Error;

use itools::{expand_file_list, new_counter, Config, HashMaster, ItoolsError, Result};

// - Different hashers
//   - Mean
//   - Diff
//   - Perceptual
//   - MD5
// - Outputter
//   - to stderr
//   - with open text
//   - with opener
//   - none
//   - json
//   - complete or just dups
// - Persistence
//   - save
//   - load
// - Progress
//   + basic
//   - none
//   - quiet mode

fn run() -> Result<()> {
    let config = Config::new()?;
    let (files, _missing) = expand_file_list(config.files)?;

    // TODO: report the missing files.
    let p = if config.show_progress {
        new_counter(files.len() as u64)
    } else {
        None
    };

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
