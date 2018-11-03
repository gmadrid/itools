extern crate clap;
extern crate console;
extern crate image;
extern crate img_hash;
extern crate indicatif;
#[macro_use]
extern crate lazy_static;
extern crate subprocess;
extern crate walkdir;

mod config;
mod fileinfo;
mod hasher;
mod progress;
mod result;
mod utils;
mod walker;

pub use config::Config;
pub use hasher::HashMaster;
pub use progress::new_counter;
pub use result::{ItoolsError, Result};
pub use utils::bool_to_option;
pub use walker::expand_file_list;
