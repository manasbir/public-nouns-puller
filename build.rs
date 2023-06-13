use std::{fs, path::Path};

fn main() {
    if !Path::new("./nouns").exists() {
    fs::create_dir("./nouns").expect("failed to create dir");
    }
}