mod file_help;
mod side_models;

use side_models::{Word, Language};
use std::fs::File;
use std::error::Error;

const FILE_TYPE: &'static str = "csv";

pub struct Manager {
    pub file: File,
    pub langs: Vec<Language>,
    pub words: Vec<Word>,
    pub def_lang: i32
} impl Manager {
    pub fn new(direc: &str, name: &str) -> Manager {
        Manager {
            file: file_help::open(
                direc.to_string(),
                name.to_string(),
                FILE_TYPE.to_string()
            ),
            langs: Vec::new(),
            words: Vec::new(),
            // Default lang is 0 (0 => none)
            def_lang: 0
        }
    }

    pub fn parse(&mut self) {
        let mut content = file_help::read(&mut self.file);
        let lines= content.lines().collect::<Vec<&str>>();

        let first_row= lines.first().unwrap().split(",").collect::<Vec<&str>>();

        // NOTE: Care about performance for parsing words
        // Old method:
        // Going through all rows and getting the value at position of current language
        // New method:
        // Going through the rows, then languages

        //Problem with IDs
        // NOTE: ID for word is current loop position, not actual ID from table
        for current_row in 1..lines.len() {
            let row_vec: Vec<&str> = lines[current_row].split(",").collect();
            let word_index: i32 = match row_vec.first().unwrap().trim().parse() {
                Ok(i) => i,
                Err(why) => panic!("Not an index for words in row {}. Error: {}", current_row, why.description())
            };
            for lang_id in 1..first_row.len() {
                match row_vec.get(lang_id) {
                    Some(val) => { self.words.push(Word::new(word_index, lang_id as i32 - 1, val));
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

    // Return word reference of current def. lang. at index
    pub fn get_word(&self, word_id: i32) -> &Word {
        for word in &self.words {
                if word.lang_id == self.def_lang - 1 && word.id == word_id {
                    return word
                }
        }
        panic!("Couldn't find a word with id {} at set def. lang. {}", word_id, self.def_lang);
    }

}
