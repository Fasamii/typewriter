// General TODO:
// - store how long user is pressing key - to allow improvement in that field (dwell_time)
// - store how long does it take for user to fly from one key to another - for improvement
// - store bigram (2) and trigram (3) of keys fly time / accuracy - to customize train text and improvement

mod plane;
mod store;
mod terminal;
mod text;

fn main() {
    let config_path = store::files::get_config_dir().unwrap();
    println!("config dir : {:?}", &config_path);

    let mut profile: store::profile::Profile = match store::profile::Profile::load(&config_path) {
        Ok(profile) => profile,
        Err(_) => store::profile::Profile::new(),
    };

    profile.keys.insert('a', store::profile::CharStats::new());
    profile.keys.get_mut(&'a').unwrap().count = 12;

    profile
        .bigrams
        .insert(('a', 'b'), store::profile::BigramStats::new());
    profile
        .bigrams
        .insert(('b', 'b'), store::profile::BigramStats::new());
    profile
        .bigrams
        .insert(('b', 'c'), store::profile::BigramStats::new());

    println!("profile : {profile:?}");
    println!(
        "accuracy of 'a' : {}",
        profile.keys.get(&'a').unwrap().accuracy()
    );

    profile.save(&config_path).unwrap();
}
