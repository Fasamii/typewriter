mod plane;
mod store;
mod terminal;
mod text;

fn main() {
    let config_path = store::files::get_config_dir().unwrap();
    println!("config dir : {:?}", config_path);
    let _ = store::files::write_char(&vec![('a', 123), ('b', 111), ('c', 89)], config_path);
}
