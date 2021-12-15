use std::{error::Error, path::Path};

pub trait InputReader<T> {
    fn string_to_vector(input_str: String) -> Vec<T>;
    fn from_file(input_filepath: &Path) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}
