use std::collections::HashMap;
use std::path::PathBuf;

use bk_tree::Metric;
use img_hash::ImageHash;

use super::fileinfo::FileInfo;

#[derive(Clone, Copy, Debug)]
pub enum SearchType {
    SHA2,
    MEAN(u8),
    GRAD(u8),
    DCT(u8),
}

impl Default for SearchType {
    fn default() -> SearchType {
        SearchType::SHA2
    }
}

// Steps
// 1. Build reverse index from hash to ImageList.
// 2. If distance == 0, find the index and spew.
// 3. Otherwise, build the bk-tree, and use it to search.
// 4. Spew results, preferably in distance order.

impl SearchType {
    fn distance(&self) -> u64 {
        use self::SearchType::*;
        match *self {
            MEAN(d) => d as u64,
            GRAD(d) => d as u64,
            DCT(d) => d as u64,
            SHA2 => 0u64,
        }
    }

    fn get_hash<'a>(&self, fi: &'a FileInfo) -> &'a str {
        use self::SearchType::*;
        match *self {
            SHA2 => &fi.sha2_hash,
            MEAN(_) => &fi.a_hash,
            GRAD(_) => &fi.d_hash,
            DCT(_) => &fi.p_hash,
        }
    }

    pub fn find_dups(
        &self,
        files: Vec<PathBuf>,
        fileinfos: HashMap<PathBuf, FileInfo>,
    ) -> Vec<Matches> {
        let index = self.build_reverse_index(&mut fileinfos.values());

        if self.distance() == 0 {
            self.find_exact_distance(files, index, fileinfos)
        } else {
            panic!("WHAT!")
        }
    }

    fn find_exact_distance(
        &self,
        files: Vec<PathBuf>,
        index: HashMap<String, Vec<PathBuf>>,
        fileinfos: HashMap<PathBuf, FileInfo>,
    ) -> Vec<Matches> {
        files.iter().fold(Vec::default(), |mut matches, filename| {
            if let Some(fi) = fileinfos.get(filename) {
                let hash = self.get_hash(fi);

                if let Some(matched_files) = index.get(hash) {
                    // TODO: Remove the filename from the matched files.
                    if matched_files.len() > 1 {
                        matches.push(Matches {
                            filename: filename.to_owned(),
                            matched_files: matched_files.to_owned(),
                        });
                    }
                }
            }
            matches
        })
    }

    fn build_reverse_index<'a, T>(
        &self,
        fileinfos: &'a mut T, // Vec<FileInfo>
    ) -> HashMap<String, Vec<PathBuf>>
    where
        T: Iterator<Item = &'a FileInfo>,
    {
        fileinfos.by_ref().fold(
            HashMap::<String, Vec<PathBuf>>::default(),
            |mut index, fi| {
                let key = self.get_hash(fi);
                index
                    .entry(key.to_string())
                    .or_insert_with(|| Vec::default())
                    .push(fi.filename.clone());
                index
            },
        )
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Matches {
    pub filename: PathBuf,
    pub matched_files: Vec<PathBuf>,
}

struct HammingDistance;

impl Metric<ImageHash> for HammingDistance {
    fn distance(&self, a: &ImageHash, b: &ImageHash) -> u64 {
        a.dist(b) as u64
    }
}
