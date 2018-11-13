extern crate itools;

use std::error::Error;

use itools::neardups::{
    bool_to_option, expand_file_list, new_counter, output::Output, Config, Hasher, ItoolsError,
    PersistedCache, Result,
};

fn run() -> Result<()> {
    let config = Config::new()?;

    // If we fail to load the cached file, report an error to avoid overwriting data.
    // If the file doesn't exist, then go ahead and create a brand new one.
    let mut cache = if !config.cache_file.exists() {
        PersistedCache::new()
    } else {
        match PersistedCache::load(&config.cache_file) {
            Ok(c) => c,
            Err(e) => return Err(e),
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
        let matches = config.search.find_dups(files, fileinfo);
        config.output.output(matches);
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
