extern crate image;
extern crate itools;
extern crate sha2;

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
    let (files, _missing) = expand_file_list(config.files)?;

    Machine::run(files).join();

    // TODO: report the missing files.
    // TODO: move this into run, maybe?
    //    let num_files = files.len();
    //    HashMaster::new(files).run(bool_to_option(config.show_progress, || {
    //        new_counter(num_files as u64)
    //    }));

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
