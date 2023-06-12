use std::{fs, path::Path};

fn main() {
    if Path::new("/Users/manas/Documents/projectrs/public-nouns-puller/nouns").exists() == false {
    fs::create_dir("/Users/manas/Documents/projectrs/public-nouns-puller/nouns").expect("failed to create dir");
    }
}