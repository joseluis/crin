use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use toml_edit::Document;

use colored::*;

use super::{ORGANIZATION, APPNAME, CONFIGNAME};

lazy_static! {
    static ref SETTINGS: RwLock<Document> = RwLock::new(Document::new());
}

mod lists; pub use self::lists::Lists;


pub struct Settings {}


impl Settings {

    pub fn dir() -> PathBuf {
        directories::ProjectDirs::from("rs", ORGANIZATION, APPNAME)
        .expect("Unable to retrieve app config directory").config_dir().to_owned()
    }

    /// Reads the configuration from the file and the environment
    pub fn read() {
        let mut settings = SETTINGS.write().unwrap();

        // The configuration file path is [OS dependant](https://crates.io/crates/directories)
        let config_path = Self::dir();
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
            let mut toml = String::new();
            File::open(config_file)
                .expect("Error: couldn't open configuration file.")
                .read_to_string(&mut toml)
                .expect("Error: couldn't read configuration file.");

            let toml_doc = toml.parse::<Document>().expect("invalid toml");
            let _ = std::mem::replace(&mut *settings, toml_doc);

        } else {
            // println!("No configuration file exists. Creating a new one...");
            // if let Err(e) = fs::write(config_file, "") {
            //     println!("Error: failed to create a new configuration file:\n{}", e);
            // }
        }
    }

    /// Writes the updated settings to the configuration file
    pub fn write() {

        let config_file = Self::dir().join(CONFIGNAME);
        let settings = SETTINGS.read().unwrap();

        if let Ok(mut file) = fs::OpenOptions::new().write(true).create(true).open(config_file) {
            file.set_len(0).unwrap();

            file.write(settings.to_string().as_bytes())
                .expect("Error: couldn't write to the configuration file.");
        } else {
            println!("{}", "Error: Couldn't save the configuration.".bright_red());
        }
    }

}
