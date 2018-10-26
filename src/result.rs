use std::result;

pub type Result<T> = result::Result<T, ItoolsError>;

#[derive(Clone,Debug)]
pub struct ItoolsError;

