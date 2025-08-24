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
