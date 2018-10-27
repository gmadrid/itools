use std::io;
use std::result;

use clap;
use image;
use walkdir;

pub type Result<T> = result::Result<T, ItoolsError>;

#[derive(Debug)]
pub enum ItoolsError {
    InvalidState(&'static str),
    UsageError(&'static str),

    Clap(clap::Error),
    Image(image::ImageError),
    IO(io::Error),
    WalkDir(walkdir::Error),
}

impl From<clap::Error> for ItoolsError {
    fn from(err: clap::Error) -> ItoolsError {
        ItoolsError::Clap(err)
    }
}

impl From<image::ImageError> for ItoolsError {
    fn from(err: image::ImageError) -> ItoolsError {
        ItoolsError::Image(err)
    }
}

impl From<io::Error> for ItoolsError {
    fn from(err: io::Error) -> ItoolsError {
        ItoolsError::IO(err)
    }
}

impl From<walkdir::Error> for ItoolsError {
    fn from(err: walkdir::Error) -> ItoolsError {
        ItoolsError::WalkDir(err)
    }
}
