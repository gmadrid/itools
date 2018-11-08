extern crate itools;

use std::error::Error;
use std::path::PathBuf;

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
    let filename = PathBuf::from("test_output.yaml");

    // If we fail to load the cached file, create a new cache.
    // TODO: we may want to make this an option to avoid overwriting data.
    let mut cache = match PersistedCache::load(&filename) {
        Ok(c) => c,
        Err(_) => PersistedCache::new(),
    };

    // TODO: report the missing files.
    let (files, _missing) = expand_file_list(config.files)?;

    // TODO: add the progress meter back in.
    let (hasher, agg_rx) = Hasher::run(files);

    cache.run(filename, agg_rx);

    // Join the cache first to ensure that it's done before its senders are dropped.
    cache.join();
    hasher.join();

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
