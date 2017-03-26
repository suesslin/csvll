// Copyright 2017 Lukas A. Mueller.  All rights reserved.
// Use of this source code is governed by a MIT style
// license that can be found in the LICENSE file.

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

pub fn create(dir: String, name: String, extension: String) -> File {
    match File::create(Path::new(&format!("{}/{}.{}", dir, name, extension))) {
        Ok(file) => file,
        Err(why) => panic!("{}", why.description())
    }
}

pub fn open(dir: String, name: String, extension: String) -> File {
    match File::open(Path::new(&format!("{}/{}.{}", dir, name, extension))) {
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

pub fn delete(dir: String, name: String, extension: String) {
    fs::remove_file(format!("{}/{}.{}", dir, name, extension));
}
