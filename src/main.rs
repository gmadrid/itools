extern crate itools;

use std::error::Error;

use itools::{
    expand_file_list, Config, ItoolsError,
    Machine,
    Result,
};

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

fn run() -> Result<()> {
    let config = Config::new()?;

    // TODO: report the missing files.
    let (files, _missing) = expand_file_list(config.files)?;

    // TODO: add the progress meter back in.
    Machine::run(files).join();

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
