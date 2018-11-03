pub fn bool_to_option<T, F: FnOnce() -> T>(b: bool, f: F) -> Option<T> {
    if b { Some(f()) } else { None }
}
