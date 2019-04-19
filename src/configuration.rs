use std::fs;
//use std::fs::File;
//use std::io::Write;
//use std::path::{Path,PathBuf};
use std::path::{PathBuf};
use std::sync::RwLock;

use config::*;
use colored::*;

use super::{ORGANIZATION, APPNAME, CONFIGNAME};

lazy_static! {
    static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}

pub fn config_dir() -> PathBuf {
    directories::ProjectDirs::from("rs", ORGANIZATION, APPNAME)
    .expect("Unable to retrieve app config directory").config_dir().to_owned()
}

/// Reads the configuration from the file and the environment
pub fn config_read() {
    let mut settings = SETTINGS.write().unwrap();

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
}


pub struct Lists {}

impl Lists {

    /// Checks if a list already exists
    pub fn is(list: &str) -> bool {
        let settings = SETTINGS.read().unwrap();

        if let Ok(lists) = settings.get_table("lists") {
            lists.contains_key(list)
        } else {
            false
        }
    }

    /// Shows the crates contained in the list
    pub fn show(list: &str, plain: bool) -> Option<String> {
        let settings = SETTINGS.read().unwrap();

        if let Ok(lists) = settings.get_table("lists") {
            if let Some(value) = lists.get(list) {
                if let Ok(array) = value.to_owned().into_array() {
                    let mut crates_str = "".to_string();
                    for value in array {
                        if let Ok(v) = value.into_str() {
                            if plain {
                                crates_str = format!("{}, {}", crates_str, v.green());
                            } else {
                                crates_str = format!("{} {}", crates_str, v);
                            }
                        }
                    }
                    return Some(crates_str[1..].trim().to_string()); // remove leading comma
                }
            }
        }
        None
    }

    /// Returns the number of crates in a list
    pub fn quantity(list: &str) -> usize {
        let settings = SETTINGS.read().unwrap();
        if let Ok(lists) = settings.get_table("lists") {
            if let Some(value) = lists.get(list) {
                if let Ok(array) = value.to_owned().into_array() {
                    return array.len();
                }
            }
        }
        0
    }


    pub fn show_lists(recursive: bool) {
        let settings = SETTINGS.read().unwrap();

        if let Ok(lists) = settings.get_table("lists") {
            if recursive {
                for (list_name, value) in lists {
                        if let Ok(array) = value.into_array() {
                            println!("{}", list_name);
                            for crate_name in array {
                                println!("\t{}", crate_name);
                            }
                        }
                }
            } else {
                //let lists_names: Vec<&String> = lists.keys().collect();
                //println!("{:?}", lists_names[0]);

                let mut lists_str = "".to_string();
                for (list, _) in lists {
                    lists_str = format!("{}, {} {}",
                        lists_str, list.bright_green(),
                        format!("({})", Self::quantity(&list)).cyan()
                    );
                }
                println!("Your lists: {}", lists_str[1..].trim());
            }
        }

    }
}



/*
// Default configuration settings
pub fn config_default_settings() {

    let settings = SETTINGS.write().unwrap();

    // TODO: save default values
    //default_settings.set

    let mut settings = SETTINGS.lock().unwrap();
    if let Err(e) = settings.merge(default_conf)) {
        println!("Error: failed to merge the default  file:\n{}", e);
    }
}
*/

/*
pub fn config_write() {

    let path = Path::new("test.toml");
    let settings = SETTINGS.read().unwrap();

    if let Ok(mut file) = fs::OpenOptions::new().write(true).open(path) {
        file.set_len(0).unwrap();
        for (key, value) in settings.collect().unwrap() {
            file.write_all(
                format!(
                    "{}=\"{}\"\n",
                    key,
                    value.into_str().unwrap()
                ).as_bytes()
            ).unwrap();
        }
    }

}
*/




