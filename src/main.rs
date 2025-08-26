use std::collections::{self, HashMap};

mod plane;
mod store;
mod terminal;
mod text;

fn main() {
    let config_path = store::files::get_config_dir().unwrap();
    println!("config dir : {:?}", &config_path);
    let _ = store::files::write_chars(
        vec![('a', (123, 12)), ('b', (1, 2)), ('c', (1, 2))],
        &config_path,
    );
    let mut kp: HashMap<char, (i8, i8)> = std::collections::HashMap::new();
    kp.insert('a', (1, 2));
    kp.insert('b', (2, 3));
    kp.insert('c', (3, 4));
    kp.insert('g', (4, 5));

    let _ = store::files::write_chars(kp, &config_path);

    let mut chars_data = store::files::read_chars(&config_path).unwrap();
    chars_data.shrink_to_fit();
    println!("chars data is : {chars_data:?}");
}
