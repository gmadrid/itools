use std::collections::HashMap;
use std::path::{Path, PathBuf};

use image::{self, GenericImageView, Pixel};
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
        let mut map: HashMap<u64, &Path> = HashMap::with_capacity(image_count);

        println!("Computing hash for {} images.", image_count);
        let mut count = 0usize;
        for file in &self.files {
            let sampled = self.decode(file)?;
            let hash = self.compute_hash(sampled)?;

            if map.contains_key(&hash) {
                println!("MATCH: {}", map.get(&hash).unwrap().to_str().unwrap());
                println!("       {}", file.to_str().unwrap());
            }
            map.insert(hash, file);

            count += 1;
            if count % 10 == 0 || count == image_count {
                println!("Computed {}/{} hashes.", count, image_count);
            }
        }
        Ok(())
    }

    fn compute_hash(&self, sampled: image::DynamicImage) -> Result<u64> {
        // Computing average hash

        let mut total = 0u16;
        for pixel in sampled.pixels() {
            let val: u8 = pixel.2.to_luma().data[0];
            total += val as u16;
        }
        let avg = total / 64;

        let mut hash = 0u64;
        for pixel in sampled.pixels() {
            let val = pixel.2.to_luma().data[0];
            hash <<= 1;
            if val as u16 > avg {
                hash |= 1;
            }
        }

        Ok(hash)
    }

    fn decode(&self, path: &PathBuf) -> Result<(image::DynamicImage)> {
        let im = image::open(path)?;
        let sampled = im.grayscale().thumbnail_exact(8, 8);

        Ok(sampled)
    }
}

// ahash - compute average grayscale, then set pixels to 0 or 1 based
// on lower/higher than avearage

// dhash - compute 0 or 1 based on whether pixel is brighter or darker
// than pixel to right

// phash - 32x32, DCT, then take top-left 8x8 pixels and compare each
// pixel to median value.
