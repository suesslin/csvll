use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::fs;

struct Word {
    item_id: i32,
    value: String
} impl Word {
    fn new(item_id: i32, value: String) -> Word {
        Word { item_id: item_id, value: value }
    }
}

struct Language {
    id: i32,
    name: String,
    words: Vec<Word>
} impl Language {
    fn new(id: i32, name: String, words: Vec<Word>) -> Language {
        Language { id: id, name: name, words: words}
    }
}

struct Table {
    file: File,
    languages: Vec<Language>
} impl Table {
    fn new(direc: String, name: String, extension: String) -> Table {
        Table { file: open_file(direc, name, extension), languages: Vec::new() }
    }

    fn parse_langs(&mut self) {
        let mut content = read_file(&mut self.file);
        let lines: Vec<&str> = content.split("\n").collect();

        let first_row: Vec<&str> = lines[0].split(",").collect();
        let mut langs = Vec::new();

        for i in 1..first_row.len() {
            let mut words_vec: Vec<Word> = Vec::new();
            for i2 in 1..lines.len() {
                let current_line_vec: Vec<&str> = lines[i2].split(",").collect();
                words_vec.push(Word::new(
                    current_line_vec.first()
                        .unwrap()
                        .trim()
                        .parse()
                        .unwrap()
                    , current_line_vec[i].to_string())
                )
            }
            langs.push(Language::new((i as i32) - 1, first_row[i].to_string(), words_vec))
        }
        self.languages = langs;
    }
}

fn main() {
    let mut t = Table::new("..".to_string(), "new".to_string(), "csv".to_string());
    t.parse_langs();
    for lang in t.languages {
        println!("Id: {}\nName:{}\nWords:\n", lang.id, lang.name);
        for word in lang.words {
            println!("{} with {}", word.item_id, word.value);
        }
    }
}

// =============================================================================
//  Helper functions for Files
// =============================================================================

// QUESTION: Create directory of path if doesn't exist?

// Actually able to create file without using path, just format
// QUESTION: Any downsides?

fn create_file(direc: String, name: String, extension: String) -> File {
    match File::create(Path::new(&format!("{}/{}.{}", direc, name, extension))) {
        Ok(file) => file,
        Err(why) => panic!("{}", why.description())
    }
}

fn open_file(direc: String, name: String, extension: String) -> File {
    match File::open(Path::new(&format!("{}/{}.{}", direc, name, extension))) {
        Ok(file) => file,
        Err(why) => panic!("{}", why.description())
    }
}

fn write_to_file(file: &mut File, content: &String) {
    file.write_all(content.as_bytes());
}

fn read_file(file: &mut File) -> String {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(content) => content,
        Err(why) => panic!("{}", why.description())
    };
    content
}

fn delete_file(direc: String, name: String, extension: String) {
    fs::remove_file(format!("{}/{}.{}", direc, name, extension));
}
