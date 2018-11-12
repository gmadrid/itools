extern crate itools;

use std::error::Error;

use itools::neardups::output::Output;
use itools::neardups::{
    bool_to_option, expand_file_list, find_dups, new_counter, output, Config, Hasher, ItoolsError,
    PersistedCache, Result,
};

fn run() -> Result<()> {
    let config = Config::new()?;

    // If we fail to load the cached file, create a new cache.
    // TODO: we may want to make this an option to avoid overwriting data.
    let mut cache = if !config.cache_file.exists() {
        PersistedCache::new()
    } else {
        // TODO: report this error better.
        match PersistedCache::load(&config.cache_file) {
            Ok(c) => c,
            Err(_) => PersistedCache::new(),
        }
    };

    // TODO: report the missing files.
    let (files, _missing) = expand_file_list(config.files)?;

    let mut files_to_hash = Vec::new();
    for filename in &files {
        if !cache.contains_file(filename) {
            files_to_hash.push(filename.to_owned());
        }
    }

    let num_files = files_to_hash.len() as u64;
    let (hasher, agg_rx) = Hasher::run(files_to_hash);

    let pb = bool_to_option(config.show_progress, || new_counter(num_files));
    cache.run(config.cache_file, agg_rx, pb);

    hasher.join();
    let fileinfo = cache.join();

    if !config.cache_only {
        let matches = find_dups(files, fileinfo);
        output::new_text_output().output(matches);
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(ItoolsError::Clap(err)) => println!("{}", err.description()),
        Err(e) => println!("Error: {:?}", e),
    }
}
