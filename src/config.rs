use std::env;
use std::ffi::{OsStr, OsString};

use clap::{App, Arg};

use Result;

const APP_NAME: &str = "itools";

const ABOUT: &str = "Finds dup and near-dup image files.";
const AUTHOR: &str = "George Madrid <gmadrid@gmail.com>";
const VERSION: &str = "0.1.0";

const FILES_ARG_NAME: &str = "files";
const NO_AHASH_ARG_NAME: &str = "no_ahash";
const NO_DHASH_ARG_NAME: &str = "no_dhash";
const NO_PHASH_ARG_NAME: &str = "no_phash";
const NO_PROGRESS_ARG_NAME: &str = "no_progress";
const NO_SHA2_ARG_NAME: &str = "no_sha2";
const QUIET_ARG_NAME: &str = "quiet";

#[derive(Default, Debug)]
pub struct Config {
    pub files: Vec<OsString>,
    pub show_progress: bool,
}

fn build_clap_spec<'a, 'b>() -> clap::App<'a, 'b> {
    let no_progress_arg = Arg::with_name(NO_PROGRESS_ARG_NAME).long(NO_PROGRESS_ARG_NAME);
    let quiet_arg = Arg::with_name(QUIET_ARG_NAME)
        .long(QUIET_ARG_NAME)
        .short("q");
    let files_arg = Arg::with_name(FILES_ARG_NAME)
        .multiple(true)
        .takes_value(true)
        .required(true);

    App::new(APP_NAME)
        .about(ABOUT)
        .author(AUTHOR)
        .version(VERSION)
        .arg(no_progress_arg)
        .arg(quiet_arg)
        .arg(files_arg)
}

fn files_values<'a>(matches: &clap::ArgMatches<'a>) -> Vec<OsString> {
    matches
        .values_of_os(FILES_ARG_NAME)
        .unwrap() // Should be safe, since clap ensures at least one.
        .map(OsStr::to_os_string)
        .collect()
}

fn quiet_value<'a>(matches: &clap::ArgMatches<'a>) -> bool {
    matches.is_present(QUIET_ARG_NAME)
}

fn show_progress_value<'a>(matches: &clap::ArgMatches<'a>) -> bool {
    !matches.is_present(NO_PROGRESS_ARG_NAME) && !quiet_value(matches)
}

impl Config {
    pub fn new() -> Result<Config> {
        Config::new_from(env::args_os())
    }

    pub fn new_from<I, T>(itr: I) -> Result<Config>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let matches = build_clap_spec().get_matches_from_safe(itr)?;

        Ok(Config {
            files: files_values(&matches),
            show_progress: show_progress_value(&matches),
        })
    }
}

#[cfg(test)]
use ItoolsError;

#[cfg(test)]
pub const CMD_NAME: &str = "CommandNameIgnored";

#[test]
fn test_no_file_args() {
    let c = Config::new_from(vec![CMD_NAME]);
    assert!(!c.is_ok());
    let err = c.unwrap_err();
    match err {
        ItoolsError::Clap(_) => {
            // SUCCESS
        }
        other => {
            panic!("No file args has unexpected error: {:?}", other);
        }
    }
}

#[test]
fn test_file_args() {
    let c_one = Config::new_from(vec![CMD_NAME, "foo"]);
    assert!(c_one.is_ok());
    assert_eq!(c_one.unwrap().files, vec!["foo"]);

    let c_many = Config::new_from(vec![CMD_NAME, "foo", "bar", "quux"]);
    assert!(c_many.is_ok());
    assert_eq!(c_many.unwrap().files, vec!["foo", "bar", "quux"])
}
