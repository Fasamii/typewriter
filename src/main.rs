mod plane;
mod profile;
mod terminal;
mod text;

fn main() {
    println!("config dir : {:?}", profile::files::get_config_dir());
    profile::files::write_char('a', 19, profile::files::get_config_dir().unwrap());
    profile::files::write_char('b', 40, profile::files::get_config_dir().unwrap());
    profile::files::write_char('c', 30, profile::files::get_config_dir().unwrap());
}
