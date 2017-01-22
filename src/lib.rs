mod file_help;
mod side_models;

use side_models::{Word, Language};
use std::fs::File;

pub struct Manager {
    pub file: File,
    pub langs: Vec<Language>,
    pub words: Vec<Word>,
    pub def_lang: i32
} impl Manager {
    pub fn new(direc: &str, name: &str, ext: &str) -> Manager {
        Manager {
            file: file_help::open(
                direc.to_string(),
                name.to_string(),
                ext.to_string()
            ),
            langs: Vec::new(),
            words: Vec::new(),
            // Default lang is 0 (0 => none)
            def_lang: 0
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

    // Change default language for manager / app
    pub fn set_def(&mut self, lang_id: i32) {
        self.def_lang = lang_id + 1
    }

    pub fn get_def(&self) -> (&Language, Vec<&Word>) {
        // TODO: Try to make it work without temporary vec
        let mut word_vec: Vec<&Word> = Vec::new();
        for word in &self.words {
            if word.lang_id == (self.def_lang - 1) {
                word_vec.push(word)
            }
        }

        if self.def_lang != 0 {
            // NOTE: def_lang - 1, because we store at default setting + 1
            if let Some(lang) = self.langs.get((self.def_lang - 1) as usize) {
                // TODO: Return right word vec
                return (lang, word_vec)
            }  else {
                panic!("Couldn't find set default language")
            }
        } else {
            panic!("No default language set!");
        }
    }

}
