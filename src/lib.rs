extern crate clap;
extern crate image;
extern crate img_hash;
#[macro_use]
extern crate lazy_static;
extern crate subprocess;
extern crate walkdir;

mod config;
mod hasher;
mod result;
mod walker;

pub use config::Config;
pub use hasher::HashMaster;
pub use result::{ItoolsError, Result};
pub use walker::expand_file_list;
