pub struct User {
    wpm: i128,
}

pub struct HardChar {
    char: char,
    misses: i128,
}

pub struct HardWord<'a> {
    word: String,
    misses: i128,
    chars_missed: Vec<&'a HardChar>,
}

pub enum Data {
    User(User),
    Chars(Vec<HardChar>),
    Worlds,
}
