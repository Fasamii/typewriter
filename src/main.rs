mod terminal;
mod profile;
mod text;
mod plane;

fn main() {
    println!("config dir : {:?}", profile::files::get_config_dir());
}
