use std::collections::HashMap;
use std::path::PathBuf;

use image;
use img_hash::{HashType, ImageHash};
//use subprocess::{Popen, PopenConfig};

use Result;

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
        HashMaster { files }
    }

    pub fn run(self) -> Result<()> {
        let image_count = self.files.len();
        let mut map: HashMap<ImageHash, Vec<PathBuf>> = HashMap::with_capacity(image_count);

        self.files
            .into_iter()
            .enumerate()
            .flat_map(|(num, file)| {
                if (num + 1) % 10 == 0 {
                    println!("Hashing: {}/{}", num + 1, image_count);
                }
                image::open(&file).map(|im| (file, im))
            }).map(|(file, im)| (file, im.grayscale().thumbnail_exact(8, 8)))
            .map(|(file, scaled)| (file, ImageHash::hash(&scaled, 8, HashType::Mean)))
            .for_each(|(file, hsh)| {
                let v = map.entry(hsh).or_insert_with(|| Vec::default());
                v.push(file);
                if v.len() > 1 {
                    // TODO: Rewrite this with join().
                    print!("  open ");
                    for s in v {
                        print!(" {:?} ", s);
                    }
                    print!("\n");
                }
            });
        Ok(())
    }
}
