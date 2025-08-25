const CHARS_FILE_NAME: &'static str = "chars";

use std::{collections, env, fs, io, path};

pub fn get_config_dir() -> io::Result<path::PathBuf> {
    let appname = "typewriter";

    let mut candidates = Vec::new();

    if let Ok(dir) = env::var("XDG_CONFIG_HOME") {
        candidates.push(path::PathBuf::from(dir).join(appname));
    }
    if let Ok(home) = env::var("HOME") {
        candidates.push(path::PathBuf::from(home).join(".config").join(appname));
    }
    if let Ok(appdata) = env::var("APPDATA") {
        candidates.push(path::PathBuf::from(appdata).join(appname));
    }
    candidates.push(path::PathBuf::from(".").join(appname));

    let mut last_error: Option<io::Error> = None;

    for candidate in &candidates {
        if candidate.exists() {
            return Ok(candidate.clone());
        }

        match fs::create_dir_all(&candidate) {
            Ok(_) => return Ok(candidate.clone()),
            Err(err) => {
                last_error = Some(err);
                continue;
            }
        };
    }

    Err(last_error.unwrap_or_else(|| {
        io::Error::new(
            io::ErrorKind::Other,
            format!(
                "UN able to create config dir at any of that locations: {:?}",
                candidates
            ),
        )
    }))
}

// STUDY: that function so you can understand it better, you literally struggled to write that and
// used a lot of code from the internet, make yourself able to write that completely by yourself,
// todo list to achieve that:
// - Read the IntoIterator trait docs - to expand your knowledge about traits
// - Read entire std collections docs - to improve you data structures skills
// - Write some toy but doing something useful code for each collection - to consolidate
// PS. Future me plz do that.
pub fn write_chars<I>(new_records: I, config: &path::PathBuf) -> csv::Result<()>
where
    I: IntoIterator<Item = (char, i8)>,
{
    let file = config.join(CHARS_FILE_NAME);

    let mut char_data: collections::HashMap<char, i8> = collections::HashMap::new();

    if file.exists() {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&file)?;

        for result in reader.records() {
            let record = result?;

            if record.len() != 2 {
                // TODO: make csv fix function that will fix the csv if its integrity is broken
                todo!("Handle malformed CSV records");
            }

            if let (Some(char_str), Some(acc_str)) = (record.get(0), record.get(1)) {
                if char_str.chars().count() == 1 {
                    if let Some(parsed_char) = char_str.chars().next() {
                        if let Ok(parsed_acc) = acc_str.parse::<i8>() {
                            char_data.insert(parsed_char, parsed_acc);
                        }
                    }
                }
            }
        }
    }

    for (ch, accuracy) in new_records {
        char_data.insert(ch, accuracy);
    }

    let mut writer = csv::Writer::from_path(&file)?;
    for (character, acc) in char_data.iter() {
        writer.write_record(&[character.to_string(), acc.to_string()])?;
    }

    writer.flush()?;
    Ok(())
}

pub fn read_chars(config: &path::PathBuf) -> csv::Result<collections::HashMap<char, i8>> {
    let file = config.join(CHARS_FILE_NAME);

    let mut char_data = collections::HashMap::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)?;

    for record in reader.records() {
        let record = record?;
        if let (Some(ch), Some(acc)) = (record.get(0), record.get(1)) {
            if let Some(parsed_ch) = ch.chars().next() {
                if let Ok(parsed_acc) = acc.parse::<i8>() {
                    char_data.insert(parsed_ch, parsed_acc);
                }
            }
        }
    }

    Ok(char_data)
}
