use std::collections::HashSet;
use std::ffi::OsString;
use std::path::PathBuf;

use walkdir::WalkDir;

use Result;

lazy_static! {
    static ref EXTENSIONS: HashSet<OsString> = {
        vec!["gif", "jpg", "jpeg", "png"]
            .into_iter()
            .map(|s| OsString::from(s))
            .collect::<HashSet<OsString>>()
    };
}

pub fn expand_file_list(files: Vec<OsString>) -> Result<(Vec<PathBuf>, Vec<PathBuf>)> {
    let (existing, missing): (Vec<PathBuf>, Vec<PathBuf>) = files.into_iter()
        .map(|osstr| PathBuf::from(osstr))
        .partition(|path| path.exists());

    let (directories, mut files): (Vec<PathBuf>, Vec<PathBuf>) = existing.into_iter()
        .partition(|path| path.is_dir());

    for dir in directories {
        for entry in WalkDir::new(dir) {
            if let Ok(e) = entry {
                let path = e.path();
                if let Some(ext) = path.extension() {
                    // TODO: Deal with upper-case extensions.
                    if EXTENSIONS.contains(ext) {
                        files.push(path.into());
                    }
                }
            }
        }
    }

    Ok((files, missing))
}
