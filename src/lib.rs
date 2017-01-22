mod file_help;
mod side_models;

use side_models::{Word, Language};

use std::fs::File;

pub struct Manager {
    pub file: File,
    pub langs: Vec<Language>,
    pub words: Vec<Word>
} impl Manager {
    pub fn new(direc: &str, name: &str, ext: &str) -> Manager {
        Manager {
            file: file_help::open(
                direc.to_string(),
                name.to_string(),
                ext.to_string()
            ),
            langs: Vec::new(),
            words: Vec::new()
        }
    }

    pub fn parse(&mut self) {
        let mut content = file_help::read(&mut self.file);
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
