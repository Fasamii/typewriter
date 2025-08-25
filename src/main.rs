use std::collections::{self, HashMap};

mod plane;
mod store;
mod terminal;
mod text;

fn main() {
    let config_path = store::files::get_config_dir().unwrap();
    println!("config dir : {:?}", &config_path);
    let _ = store::files::write_chars(vec![('a', 123), ('b', 111), ('c', 89)], &config_path);
    let mut kp: HashMap<char, i8> = std::collections::HashMap::new();
    kp.insert('a', 12);
    kp.insert('b', 12);
    kp.insert('c', 14);
    kp.insert('g', 14);

    let _ = store::files::write_chars(kp, &config_path);

    let mut chars_data = store::files::read_chars(&config_path).unwrap();
    chars_data.shrink_to_fit();
    println!("chars data is : {chars_data:?}");
}
