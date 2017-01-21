pub struct Word {
    pub id: i32,
    pub lang_id: i32,
    pub val: String
} impl Word {
    pub fn new(id: i32, lang_id: i32, val: &str) -> Word {
        Word { id: id, lang_id: lang_id, val: val.to_string() }
    }
}

pub struct Language {
    pub id: i32,
    pub name: String
} impl Language {
    pub fn new(id: i32, name: &str) -> Language {
        Language { id: id, name: name.to_string() }
    }
}
