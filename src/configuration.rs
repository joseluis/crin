use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use config::*;

use super::{ORGANIZATION, APPNAME, CONFIGNAME};

lazy_static! {
    static ref SETTINGS: Mutex<Config> = Mutex::new(Config::default());
}

pub fn config_dir() -> PathBuf {
    directories::ProjectDirs::from("rs", ORGANIZATION, APPNAME)
    .expect("Unable to retrieve app config directory").config_dir().to_owned()
}

/// Reads the configuration from the file and the environment
pub fn config_read() {
    let mut settings = SETTINGS.lock().unwrap();

    // The configuration file path is [OS dependant](https://crates.io/crates/directories)
    let config_path = config_dir();
    let config_file = config_path.join(CONFIGNAME);

    if config_path.exists() {
        if !config_path.is_dir() {
            panic!("configuration path exists, but is not a directory:\n{}",
                   config_path.display());
        }
    } else {
        // TODO don't create it by default
        if let Err(e) = fs::create_dir_all(config_path) {
            println!("Error: failed to create the configuration directory:\n{}", e);
        }
    }

    // Add in settings from the configuration file
    if config_file.exists() {
        if let Err(e) = settings.merge(config::File::from(config_file.clone())) {
            println!("Error: failed to parse the configuration file:\n{}", e);
        }
    } else {
        // TODO don't create it by default
        println!("No configuration file exists. Creating a new one...");
        if let Err(e) = fs::write(config_file, "") {
            println!("Error: failed to create a new configuration file:\n{}", e);
        }
    }

    // Add in settings from the environment (with a prefix of CRIN)
    // E.g.: `CRIN_DEFAULT_STYLE=default_light crin`
    if let Err(e) = settings.merge(config::Environment::with_prefix(&APPNAME)) {
        println!("Failed parsing the environment settings:\n{}", e);
    }

    //println!("SETTINGS:\n--------\n{}, {:?}", APPNAME, settings); // DEBUG
}



pub fn config_write() {

}




