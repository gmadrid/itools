use std::collections::HashMap;
use std::path::PathBuf;

use bk_tree::Metric;
use img_hash::ImageHash;

use super::fileinfo::FileInfo;

#[derive(Debug)]
pub enum DynamicSearch{
    SHA2,
    MEAN,
    GRAD,
    DCT,
}

impl Default for DynamicSearch {
    fn default() -> DynamicSearch {
        DynamicSearch::DCT
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Matches {
    pub filename: PathBuf,
    pub matched_files: Vec<PathBuf>,
}

// Steps
// 1. Build reverse index from hash to ImageList.
// 2. If distance == 0, find the index and spew.
// 3. Otherwise, build the bk-tree, and use it to search.
// 4. Spew results, preferably in distance order.

pub fn find_dups(files: Vec<PathBuf>, fileinfos: HashMap<PathBuf, FileInfo>) -> Vec<Matches> {
    // TODO: can I get rid of the clones?

    let index = (&fileinfos).values().fold(
        HashMap::<String, Vec<PathBuf>>::default(),
        |mut state, fi| {
            let key = &fi.sha2_hash;
            state
                .entry(key.to_string())
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
                let sha2 = &fi.sha2_hash;

                if let Some(matched_files) = index.get(sha2) {
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

struct HammingDistance;

impl Metric<ImageHash> for HammingDistance {
    fn distance(&self, a: &ImageHash, b: &ImageHash) -> u64 {
        a.dist(b) as u64
    }
}
