use std::collections::HashMap;
use std::path::PathBuf;

pub trait Output {
    fn output(hsh: HashMap<u64, Vec<PathBuf>>);
}
