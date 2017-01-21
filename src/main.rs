use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::fs;

struct Word {
    id: i32,
    lang_id: i32,
    val: String
} impl Word {
    fn new(id: i32, lang_id: i32, val: &str) -> Word {
        Word { id: id, lang_id: lang_id, val: val.to_string() }
    }
}

struct Language {
    id: i32,
    name: String
} impl Language {
    fn new(id: i32, name: &str) -> Language {
        Language { id: id, name: name.to_string() }
    }
}

struct Table {
    file: File,
    langs: Vec<Language>,
    words: Vec<Word>
} impl Table {
    fn new(direc: &str, name: &str, ext: &str) -> Table {
        Table {
            file: open_file(
                direc.to_string(),
                name.to_string(),
                ext.to_string()
            ),
            langs: Vec::new(),
            words: Vec::new()
        }
    }

    fn parse_langs(&mut self) {
        let mut content = read_file(&mut self.file);
        let lines: Vec<&str> = content.split("\n").collect();

        let first_row: Vec<&str> = lines.first().unwrap().split(",").collect();

        // NOTE: Care about performance for parsing words
        // Old method:
        // Going through all rows and getting the value at position of current language
        // New method:
        // Going through the rows, then languages

        // Word parsing
        // NOTE: IDs must go -1, because starts counting at 1 (1 should be 0)
        for word_i in 1..lines.len() {
            let row_vec: Vec<&str> = lines[word_i].split(",").collect();
            for lang_id in 1..first_row.len() {
                match row_vec.get(lang_id) {
                    Some(val) => { self.words.push(Word::new(word_i as i32 - 1, lang_id as i32 - 1, val));
                                    // println!("{}", val)
                                 },
                    None => println!("Hey")
                }
            }
        }

        // Language parsing
        for lang_i in 1..first_row.len() {
            self.langs.push(Language::new(lang_i as i32 - 1, first_row[lang_i]));
        }
    }
}

fn main() {
    let mut t = Table::new("..", "table", "csv");
    t.parse_langs();
    // for word in t.words {
    //     println!("at {}: {} (lang: {})", word.id, word.val, word.lang_id)
    // }

    let mut pl: Vec<Word> = Vec::new();

    for word in t.words {
        if word.lang_id == 1 {
            pl.push(word)
        }
    }

    for p in pl {
        println!("{}", p.val)
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
