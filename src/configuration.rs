use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path,PathBuf};
use std::sync::RwLock;

use config::*;
use colored::*;

use super::{ORGANIZATION, APPNAME, CONFIGNAME};

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
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
            if let Err(e) = settings.merge(config::File::from(config_file.clone())) {
                println!("Error: failed to parse the configuration file:\n{}", e);
            }
        } else {
            // println!("No configuration file exists. Creating a new one...");
            // if let Err(e) = fs::write(config_file, "") {
            //     println!("Error: failed to create a new configuration file:\n{}", e);
            // }
        }

        // Add in settings from the environment (with a prefix of CRIN)
        // E.g.: `CRIN_DEFAULT_STYLE=default_light crin`
        if let Err(e) = settings.merge(config::Environment::with_prefix(&APPNAME)) {
            println!("Failed parsing the environment settings:\n{}", e);
        }
    }

    /// Writes the updated settings to the configuration file
    pub fn write() {

        let config_file = Self::dir().join(CONFIGNAME);
        let settings = SETTINGS.read().unwrap();


        if let Ok(mut file) = fs::OpenOptions::new().write(true).create(true).open(config_file) {
            file.set_len(0).unwrap();

            // THE HEADER
            file.write(b"# crin configuration file\n#\n").unwrap();
            file.write(b"# https://crates.io/crates/crin\n").unwrap();
            file.write(b"################################\n").unwrap();

            // THE LISTS
            file.write(b"\n[lists]\n").unwrap();

            if let Ok(lists) = settings.get_table("lists") {
                for (list, value) in lists {
                    // list name
                    file.write(format!("{} = ", list).as_bytes()).unwrap();
                    let mut crates = vec![];

                    // list contents
                    let mut array_str = "".to_string();
                    if let Ok(array) = value.into_array() {
                        for crat in &array {
                            crates.push(crat.to_owned().into_str().unwrap());
                        }
                        // the crates contained
                        if !array.is_empty() {
                            array_str = super::commify(
                                crates.iter().map(AsRef::as_ref).collect(), "'", None, None);
                        }
                    }
                    file.write(format!("[ {} ]\n", &array_str).as_bytes()).unwrap();
                }
            }
        } else {
            println!("{}", "Error: Couldn't save the configuration.".bright_red());
        }
    }

}
