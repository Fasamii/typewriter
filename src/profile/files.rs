use std::{env, fs, io, path};

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

// TODO: half of the code is hallucinated by that fucking llm read the docs and do that or fucking
// struggle with that fucked llm
pub fn write_char(ch: char, accuracy: i8, cfg: path::PathBuf) -> csv::Result<()> {
    let file = cfg.join("chars");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(&file)?;

    let mut records: Vec<csv::StringRecord> = reader
        .records()
        .filter_map(Result::ok) // drop Errs, keep Ok(record)
        .collect();

    let mut changed = false;
    for i in 0..records.len() {
        let record = &records[i];
        if let Some(elem) = record.get(0) {
            if elem.chars().count() == 1 && elem.chars().next() == Some(ch) {
                // Build a new record with the modified second field
                let mut new_record = csv::StringRecord::new();
                for (j, field) in record.iter().enumerate() {
                    if j == 1 {
                        new_record.push_field(&accuracy.to_string());
                    } else {
                        new_record.push_field(field);
                    }
                }
                changed = true;
                records[i] = new_record;
            }
        }
    }

    if !changed {
        let mut new_record = csv::StringRecord::new();
        new_record.push_field(ch.to_string().as_str());
        new_record.push_field(accuracy.to_string().as_str());
        records.push(new_record);
    }

    let mut writer = csv::Writer::from_path(&file)?;
    for record in records {
        writer.write_record(&record)?;
    }
    writer.flush()?;

    Ok(())
}
