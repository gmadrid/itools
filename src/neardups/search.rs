use std::collections::HashMap;
use std::path::PathBuf;

use super::fileinfo::FileInfo;

#[derive(Debug, Default)]
pub struct Matches {
    pub filename: PathBuf,
    pub matched_files: Vec<PathBuf>,
}

pub fn find_dups(files: Vec<PathBuf>, fileinfos: HashMap<PathBuf, FileInfo>) -> Vec<Matches> {
    // TODO: can I get rid of the clones?

    let index = (&fileinfos).values().fold(
        HashMap::<String, Vec<PathBuf>>::default(),
        |mut state, fi| {
            let key = fi.sha2_hash.clone().unwrap();
            state
                .entry(key)
                .or_insert_with(|| Vec::default())
                .push(fi.filename.clone());
            state
        },
    );

    let matches = (&files)
        .into_iter()
        .fold(Vec::<Matches>::default(), |mut state, filename| {
            // Look up sha2 for each file.
            if let Some(fi) = fileinfos.get(filename) {
                let sha2 = fi.sha2_hash.clone().unwrap();

                if let Some(matched_files) = index.get(&sha2) {
                    // TODO: remove the filename from the matched files.
                    if matched_files.len() > 1 {
                        state.push(Matches {
                            filename: filename.to_owned(),
                            matched_files: matched_files.to_owned(),
                        });
                    }
                }
            }
            state
        });
    matches
}
