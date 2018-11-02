use std::env;
use std::ffi::{OsStr, OsString};

use clap::{App, Arg};

use Result;

const FILES_ARG_NAME: &str = "files";
const NO_PROGRESS: &str = "no_progress";

#[derive(Default, Debug)]
pub struct Config {
    pub files: Vec<OsString>,
    pub show_progress: bool,
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
        let matches = App::new("itools")
            .version("0.1.0")
            .author("George Madrid <gmadrid@gmail.com>")
            .about("Collection of image processing tools")
            .arg(Arg::with_name(NO_PROGRESS).long(NO_PROGRESS))
            .arg(
                Arg::with_name(FILES_ARG_NAME)
                    .multiple(true)
                    .takes_value(true)
                    .required(true),
            ).get_matches_from_safe(itr)?;

        // Should be safe since Clap ensures there is at least one arg.
        let files = matches
            .values_of_os(FILES_ARG_NAME)
            .unwrap()
            .map(OsStr::to_os_string)
            .collect();

        let show_progress = !matches.is_present(NO_PROGRESS);

        Ok(Config {
            files,
            show_progress,
            ..Config::default()
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
