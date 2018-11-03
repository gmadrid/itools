use image;
use img_hash::{HashType, ImageHash};
use indicatif::ProgressBar;
// use subprocess::{Popen, PopenConfig};

use progress::Progress;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct HashMaster {
    files: Vec<PathBuf>,
}

// fn open_image(path: &Path) {
//     let _ = Popen::create(
//         &vec!["open", &path.to_str().unwrap()],
//         PopenConfig::default(),
//     );
// }

impl HashMaster {
    pub fn new(files: Vec<PathBuf>) -> HashMaster {
        HashMaster { files: files }
    }

    pub fn run(self, progress: Option<ProgressBar>) {
        let image_count = self.files.len();
        let mut map: HashMap<ImageHash, Vec<PathBuf>> = HashMap::with_capacity(image_count);

        self.files
            .into_iter()
            .flat_map(|file| image::open(&file).map(|im| (file, im)))
            .map(|(file, im)| (file, im.grayscale().thumbnail_exact(8, 8)))
            .map(|(file, scaled)| (file, ImageHash::hash(&scaled, 8, HashType::Mean)))
            .for_each(|(file, hsh)| {
                let v = map.entry(hsh).or_insert_with(|| Vec::default());
                v.push(file);
                progress.inc();
            });
    }
}
