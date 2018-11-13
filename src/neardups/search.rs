use std::collections::HashMap;
use std::path::PathBuf;

use bk_tree::{BKTree, Metric};
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

        let distance = self.distance();
        if distance == 0 {
            self.find_exact_distance(files, index, fileinfos)
        } else {
            self.find_close_matches(distance, &files, &index, &fileinfos)
        }
    }

    fn collect_matches<'a, I>(
        &self,
        filename: &PathBuf,
        close_hashes: I,
        index: &HashMap<String, Vec<PathBuf>>,
    ) -> Matches
    where
        I: Iterator<Item = (u64, &'a ImageHash)>,
    {
        let mut fns = Vec::new();
        for (_distance, hash) in close_hashes {
            let hash_str = hash.to_base64();
            let paths = &index[&hash_str];
            fns.extend(paths);
        }

        Matches {
            filename: filename.clone(),
            matched_files: fns.into_iter().map(|f| f.to_owned()).collect(),
        }
    }

    fn find_close_matches(
        &self,
        distance: u64,
        files: &Vec<PathBuf>,
        index: &HashMap<String, Vec<PathBuf>>,
        fileinfos: &HashMap<PathBuf, FileInfo>,
    ) -> Vec<Matches> {
        let bk_tree = self.build_bk_tree(&index);
        let mut vec = Vec::new();
        for file in files {
            if let Some(fi_to_find) = fileinfos.get(file) {
                let hash_to_find = self.get_hash(fi_to_find);
                let key_to_find = ImageHash::from_base64(hash_to_find).unwrap();
                let close = bk_tree.find(&key_to_find, distance);
                let matches = self.collect_matches(&file, close, index);
                if matches.matched_files.len() > 1 {
                    vec.push(matches);
                }
            }
        }
        vec
    }

    fn build_bk_tree(
        &self,
        hashes: &HashMap<String, Vec<PathBuf>>,
    ) -> BKTree<ImageHash, HammingDistance> {
        let mut bk_tree = BKTree::new(HammingDistance {});
        for key in hashes.keys() {
            let hash = ImageHash::from_base64(key).unwrap();
            bk_tree.add(hash);
        }
        bk_tree
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
