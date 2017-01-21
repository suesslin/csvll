mod file_help;
mod side_models;

use side_models::Word;
use side_models::Language;

use std::fs::File;

struct Table {
    file: File,
    langs: Vec<Language>,
    words: Vec<Word>
} impl Table {
    fn new(direc: &str, name: &str, ext: &str) -> Table {
        Table {
            file: file_help::open_file(
                direc.to_string(),
                name.to_string(),
                ext.to_string()
            ),
            langs: Vec::new(),
            words: Vec::new()
        }
    }

    fn parse_langs(&mut self) {
        let mut content = file_help::read_file(&mut self.file);
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
    for word in t.words {
        println!("at {}: {} (lang: {})", word.id, word.val, word.lang_id)
    }

    // let mut pl: Vec<Word> = Vec::new();
    //
    // for word in t.words {
    //     if word.lang_id == 1 {
    //         pl.push(word)
    //     }
    // }
    //
    // for p in pl {
    //     println!("{}", p.val)
    // }

}
