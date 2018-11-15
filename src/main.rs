extern crate itools;

use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};

use itools::neardups::{
    bool_to_option, expand_file_list, new_counter, output::Output, Config, Hasher, ItoolsError,
    PersistedCache, Result, SpinnerReader,
};

fn load_or_create_cache_file<T>(cache_file: T) -> Result<PersistedCache>
where
    T: AsRef<Path>,
{
    if !cache_file.as_ref().exists() {
        Ok(PersistedCache::new())
    } else {
        let file = File::open(cache_file)?;
        let r = SpinnerReader::new(file, "Loading cache file...");
        PersistedCache::from_reader(r)
    }
}

fn filter_files_in_cache(files: &Vec<PathBuf>, cache: &PersistedCache) -> Vec<PathBuf> {
    files
        .iter()
        .flat_map(|f| {
            if cache.contains_file(f) {
                None
            } else {
                Some(f.clone())
            }
        }).collect::<Vec<PathBuf>>()
}

fn run() -> Result<()> {
    let config = Config::new()?;

    // If we fail to load the cached file, report an error to avoid overwriting data.
    // If the file doesn't exist, then go ahead and create a brand new one.
    let mut cache = load_or_create_cache_file(&config.cache_file)?;

    // TODO: report the missing files.
    let (files, _missing) = expand_file_list(config.files)?;

    let files_to_hash = filter_files_in_cache(&files, &cache);

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
