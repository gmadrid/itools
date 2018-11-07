extern crate itools;

use std::error::Error;
use std::path::Path;

use itools::{expand_file_list, Config, Hasher, ItoolsError, PersistedCache, Result};

// - Outputter
//   - to stderr
//   - with open text
//   - with opener
//   - none
//   - json
//   - complete or just dups

fn run() -> Result<()> {
    let config = Config::new()?;

    // This filename is unused right now. TODO: this is a bug. fix it.
    let mut cache = PersistedCache::load(Path::new("xxx"))?;

    // TODO: report the missing files.
    let (files, _missing) = expand_file_list(config.files)?;

    // TODO: add the progress meter back in.
    let (hasher, agg_rx) = Hasher::run(files);

    cache.run(agg_rx);

    hasher.join();
    cache.join();

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
