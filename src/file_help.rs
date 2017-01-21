use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::fs;

// =============================================================================
//  Helper functions for Files
// =============================================================================

// QUESTION: Create directory of path if doesn't exist?

// Actually able to create file without using path, just format
// QUESTION: Any downsides?

pub fn create(direc: String, name: String, extension: String) -> File {
    match File::create(Path::new(&format!("{}/{}.{}", direc, name, extension))) {
        Ok(file) => file,
        Err(why) => panic!("{}", why.description())
    }
}

pub fn open(direc: String, name: String, extension: String) -> File {
    match File::open(Path::new(&format!("{}/{}.{}", direc, name, extension))) {
        Ok(file) => file,
        Err(why) => panic!("{}", why.description())
    }
}

pub fn write(file: &mut File, content: &String) {
    file.write_all(content.as_bytes());
}

pub fn read(file: &mut File) -> String {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(content) => content,
        Err(why) => panic!("{}", why.description())
    };
    content
}

pub fn delete(direc: String, name: String, extension: String) {
    fs::remove_file(format!("{}/{}.{}", direc, name, extension));
}
