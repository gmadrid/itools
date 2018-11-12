use std::env;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

use clap::{self, App, Arg};

use super::output::{new_no_output, new_text_output, DynamicOutput};
use super::Result;

#[derive(Default, Debug)]
pub struct Config {
    pub cache_file: PathBuf,
    pub cache_only: bool,
    pub files: Vec<OsString>,
    pub output: DynamicOutput,
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
        let matches = build_clap_spec().get_matches_from_safe(itr)?;

        Ok(Config {
            cache_file: cache_file(&matches),
            cache_only: cache_only(&matches),
            files: files_values(&matches),
            output: show_output(&matches),
            show_progress: show_progress_value(&matches),
        })
    }
}

const APP_NAME: &str = "itools";

const ABOUT: &str = "Finds dup and near-dup image files.";
const AUTHOR: &str = "George Madrid <gmadrid@gmail.com>";
const VERSION: &str = "0.1.0";

const CACHE_FILE_ARG_NAME: &str = "cache_file";
const CACHE_FILE_ENV_NAME: &str = "NDUPS_CACHE_FILE";
const CACHE_FILE_DEFAULT_VALUE: &str = "ndups_cache";
const CACHE_ONLY_ARG_NAME: &str = "cache_only";
const FILES_ARG_NAME: &str = "files";
const NO_PROGRESS_ARG_NAME: &str = "no_progress";
const QUIET_ARG_NAME: &str = "quiet";

fn build_clap_spec<'a, 'b>() -> clap::App<'a, 'b> {
    let no_progress_arg = Arg::with_name(NO_PROGRESS_ARG_NAME).long(NO_PROGRESS_ARG_NAME);
    let quiet_arg = Arg::with_name(QUIET_ARG_NAME)
        .long(QUIET_ARG_NAME)
        .short("q");
    let cache_only_arg = Arg::with_name(CACHE_ONLY_ARG_NAME)
        .long(CACHE_ONLY_ARG_NAME)
        .short("c");
    let cache_file_arg = Arg::with_name(CACHE_FILE_ARG_NAME)
        .long(CACHE_FILE_ARG_NAME)
        .short("f")
        .env(CACHE_FILE_ENV_NAME)
        .takes_value(true)
        .default_value(CACHE_FILE_DEFAULT_VALUE);
    let files_arg = Arg::with_name(FILES_ARG_NAME)
        .multiple(true)
        .takes_value(true)
        .required(true);

    App::new(APP_NAME)
        .about(ABOUT)
        .author(AUTHOR)
        .version(VERSION)
        .arg(cache_file_arg)
        .arg(cache_only_arg)
        .arg(no_progress_arg)
        .arg(quiet_arg)
        .arg(files_arg)
}

fn cache_file<'a>(matches: &clap::ArgMatches<'a>) -> PathBuf {
    matches
        .value_of_os(CACHE_FILE_ARG_NAME)
        .unwrap_or(&OsString::from(CACHE_FILE_DEFAULT_VALUE))
        .into()
}

fn cache_only<'a>(matches: &clap::ArgMatches<'a>) -> bool {
    matches.is_present(CACHE_ONLY_ARG_NAME)
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

fn show_output<'a>(matches: &clap::ArgMatches<'a>) -> DynamicOutput {
    if quiet_value(matches) {
        new_no_output()
    } else {
        new_text_output()
    }
}

#[cfg(test)]
mod testing {
    use std::ffi::OsString;
    use std::iter::Iterator;

    use super::super::result::ItoolsError;
    use super::Config;

    pub const CMD_NAME: &str = "CommandNameIgnored";

    fn make_test_config<I, T>(itr: I) -> Config
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone + From<&'static str>,
    {
        let mut vec = vec![CMD_NAME.into()];
        vec.append(&mut itr.into_iter().collect());
        vec.append(&mut vec!["foo".into(), "bar".into()]);

        Config::new_from(vec).unwrap()
    }

    #[test]
    fn test_show_output() {
        let c_show = make_test_config(Vec::<std::ffi::OsString>::new());
        assert_eq!(true, c_show.show_output);

        let c_quiet = make_test_config(vec!["--quiet"]);
        assert_eq!(false, c_quiet.show_output);
    }

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

    #[test]
    fn test_show_progress() {
        let c_default = make_test_config(std::iter::empty::<OsString>());
        assert_eq!(true, c_default.show_progress);

        let c_no_progress = make_test_config(vec!["--no_progress"]);
        assert_eq!(false, c_no_progress.show_progress);

        let c_quiet = make_test_config(vec!["--quiet"]);
        assert_eq!(false, c_quiet.show_progress);

        let c_both = make_test_config(vec!["--quiet", "--no_progress"]);
        assert_eq!(false, c_both.show_progress);
    }

    #[test]
    fn test_cache_only() {
        let c_default = make_test_config(std::iter::empty::<OsString>());
        assert_eq!(false, c_default.cache_only);

        let c_cache_only = make_test_config(vec!["--cache_only"]);
        assert_eq!(true, c_cache_only.cache_only);
    }
}
