use crate::configuration::*;
use toml_edit::{value, Value, Array, Table};
//use std::collections::HashMap;

/// Contains the methods to manage lists in the config file
pub struct Lists {}

impl Lists {

    /// Checks if a list already exists
    pub fn exists(list: &str) -> bool {
        let settings = SETTINGS.read().unwrap();

        if settings.as_table().contains_table("lists") &&
            settings["lists"].as_table()
                .expect("Error: Couldn't parse [lists] as a valid toml table.")
                .contains_key(list) {
            true
        } else {
            false
        }
    }

    /// Shows the crates contained in a list,
    /// either as plain text, or colored & separated by commas
    pub fn show(list: &str, plain: bool) -> Option<String> {

        if Self::exists(list) {
            let settings = SETTINGS.read().unwrap();

            if let Some(crates) = settings["lists"][list].as_array() {

                let mut crates_str = "".to_string();

                if crates.is_empty() {
                    return Some("".to_string());
                } else {
                    for crat in crates.iter() {
                        if let Some(c) = crat.as_str() {
                            if plain {
                                crates_str = format!("{} {}", crates_str, c);
                            } else {
                                crates_str = format!("{}, {}", crates_str, c.green());
                            }
                        } else {
                            println!("Error: couldn't parse crate name as String: \"{}\"", crat);
                        }
                    }
                    return Some(crates_str[1..].trim().to_string()); // remove leading comma
                }
            }
        } else {
            println!("List \"{}\" doesn't exist.", list.red());
        }
        None
    }

    /// Returns the number of crates in a list
    pub fn quantity(list: &str) -> usize {
        let settings = SETTINGS.read().unwrap();
        if let Some(crates) = settings["lists"][list].as_array() {
            crates.len()
        } else {
            // TODO: recreate list as empty?
            // println!("Error: invalid format. list \"{}\" is not an Array, but {}",
            //     list.red(), Self::typeof_value(settings["lists"][list].as_value()));
            // println!("With the contents: {}", settings["lists"][list].as_value().unwrap());
            0
        }
    }

    /// Adds one crate to a list
    // TODO: allow adding multiple crates
    pub fn add(list: &str, crat: &str) {
        if Self::exists(list) {

            let mut crates_vec: Vec<&str>;

            // get the list of crates
            if let Some(crates) = Self::show(list, true) {
                crates_vec = crates.split_whitespace().collect();

                if crates_vec.contains(&crat) {
                    println!("Crate \"{}\" is already in list \"{}\"",
                        crat.red(), list.bright_red())

                // } else if ... { // TODO: check if the crate is valid
                } else {
                    let mut crates_arr = Array::default();
                    crates_arr.push(crat);
                    for c in crates_vec {
                        crates_arr.push(c);
                    }

                    {
                    let mut settings = SETTINGS.write().unwrap();
                    settings["lists"][list] = value(crates_arr);
                    }
                    println!("Added crate \"{}\" to the list \"{}\"",
                         crat.green(), list.bright_green());

                    Settings::write();
                }
            } else {
                println!("The list \"{}\" is empty.", list.red());
            }

        } else {
            println!("List \"{0}\" doesn't exist. You can create it with '{1}'",
                list.bright_red(), format!("crin list new {}", list.bright_green()).bright_blue());
        }
    }

    /// Creates a new list
    pub fn new(list: &str) {
        if Self::exists(list) {
            println!("List \"{}\" already exists.", list.red());
        } else {
            {
                let mut settings = SETTINGS.write().unwrap();

                if !settings.as_table().contains_table("lists") {
                    settings.as_table_mut()["lists"] = toml_edit::Item::Table(<Table>::new());
                }
                settings["lists"][list] = value(Array::default());
            }
            Settings::write();
        }
    }

    /// Deletes an empty list
    // TODO: allow deleting multiple lists
    pub fn del(list: &str) {
        if Self::exists(list) {
            let mut changed = false;
            {
                let mut settings = SETTINGS.write().unwrap();

                if let Some(crates) = settings["lists"][list].as_array() {

                    // Only delete the list if it's empty
                    if crates.is_empty() {
                        if let Some(table) = settings["lists"].as_table_mut() {
                            println!("Deleting the empty list \"{}\".", list.bright_green());
                            table.remove(list);
                            changed = true;
                        } else {
                            println!("Error: couldn't delete the list \"{}\".", list.red());
                        }
                    } else {
                        println!("The list \"{}\" cannot be deleted because it's not empty.",
                            list.bright_red());
                    }
                } else {
                    println!("Error: invalid format. list \"{}\" is not an Array, but {}",
                        list.red(), Self::typeof_value(settings["lists"][list].as_value()));
                    println!("With the contents: {}", settings["lists"][list].as_value().unwrap());
                    // TODO: will delete if provided with force argument
                }

            }
            if changed { Settings::write(); }
        } else {
            println!("List \"{}\" doesn't exist.", list.red());
        }
    }

    /// Removes a crate from a list
    // TODO: allow deleting multiple crates, maybe receiving a clap::Values struct
    //    https://docs.rs/clap/2.33.0/clap/struct.Values.html
    pub fn rem(list: &str, crat: &str) {
        if Self::exists(list) {
            let mut crates_vec: Vec<&str>;

            // get the list of crates
            if let Some(crates) = Self::show(list, true) {

                crates_vec = crates.split_whitespace().collect();

                if crates_vec.contains(&crat) {
                    crates_vec.retain(|&x| x != crat);

                    let mut crates_arr = Array::default();
                    for c in crates_vec {
                        crates_arr.push(c);
                    }

                    {
                    let mut settings = SETTINGS.write().unwrap();
                    settings["lists"][list] = value(crates_arr);
                    }
                    println!("Removed crate \"{}\" from the list \"{}\"",
                         crat.green(), list.bright_green());

                    Settings::write();
                } else {
                    println!("Crate \"{}\" was not in list \"{}\"",
                        crat.red(), list.bright_red())
                }
            } else {
                println!("The list \"{}\" is empty.", list.red());
            }
        } else {
            println!("List \"{}\" doesn't exist.", list.red());
        }
    }

    /// Show the saved lists
    pub fn show_lists(recursive: bool) {
        let settings = SETTINGS.read().unwrap();

        if let Some(lists) = settings["lists"].as_table() {
            if lists.len() > 0 {

                let mut lists_str = "".to_string();

                if recursive {
                    // show also the contained crates
                    for (list_name, _value) in lists.iter() {
                        lists_str = format!("{}\n{} {}: {}",
                            lists_str, list_name.bright_green(),
                            format!("({})", Self::quantity(&list_name)).cyan(),
                            if let Some(crates) = Self::show(list_name, false)
                                { crates.normal() } else { "???".bright_red() }
                        );
                    }
                } else {
                    // show just the lists with their number of crates
                    for (list_name, _value) in lists.iter() {
                        lists_str = format!("{}, {} {}",
                            lists_str, list_name.bright_green(),
                            format!("({})", Self::quantity(&list_name)).cyan()
                        );
                    }
                }
                println!("Your lists:\n{}", lists_str[1..].trim());
            } else {
                println!("You have no lists. Create a new one with '{}'",
                    "crin list new <listname>".bright_blue());
            }
            
        } else {
            println!("err");
        }
    }

    /// Returns a string identifying the type of a TOML Value
    fn typeof_value(value: Option<&Value>) -> &str {
        if let Some(v) = value {
            if v.is_integer() { return "an Integer"; }
            if v.is_str() { return "a String"; }
            if v.is_float() { return "a Float"; }
            if v.is_date_time() { return "a DateTime"; }
            if v.is_bool() { return "a Boolean"; }
            if v.is_array() { return "an Array"; }
            if v.is_inline_table() { return "an InlineTable"; }
            /*
            // ISSUE: doesn't seem to work with match
            match v {
                Value::Integer(_) => "an Integer",
                Value::String(_) => "a String",
                Value::Float(_) => "a Float",
                Value::DateTime(_) => "a DateTime",
                Value::Boolean(_) => "a Boolean",
                Value::Array(_) => "an Array",
                Value::InlineTable(_) => "an InlineTable",
            };
            */
        }
        "<unknown>"
    }

}

