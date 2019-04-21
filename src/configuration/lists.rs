use crate::configuration::*;
use std::collections::HashMap;

pub struct Lists {}

impl Lists {

    /// Checks if a list already exists
    pub fn exists(list: &str) -> bool {
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

                    if array.is_empty() {
                        return Some("".to_string());
                    } else {
                        for value in array {
                            if let Ok(v) = value.into_str() {
                                if plain {
                                    crates_str = format!("{} {}", crates_str, v);
                                } else {
                                    crates_str = format!("{}, {}", crates_str, v.green());
                                }
                            }
                        }
                        return Some(crates_str[1..].trim().to_string()); // remove leading comma
                    }
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

    /// Adds one crate to the list
    // TODO: allow adding multiple crates
    pub fn add(list: &str, crat: &str) {
        if Self::exists(list) {

            let mut crates_vec = vec![""];
            if let Some(crates) = Self::show(list, true) {
                crates_vec = crates.split_whitespace().collect();
                if crates_vec.contains(&crat) {
                    println!("Crate \"{}\" is already in list \"{}\"",
                        crat.red(), list.bright_red())
                } else {
                    // TODO: allow multiple crates
                    //println!("{:?}", crat); // DEBUG

                    // TODO: check if the crate is valid (call fn)
                    crates_vec.push(crat);

                    {
                    let mut settings = SETTINGS.write().unwrap();
                    settings.set(&format!("lists.{}", list), crates_vec).unwrap();
                    }
                    println!("Added crate \"{}\" to the list \"{}\"",
                         crat.green(), list.bright_green());

                    Settings::write();
                }
            }
            // println!("{}", Self::show(list, false).unwrap()); // DEBUG

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
                let empty_list: Vec<String> = vec![];
                settings.set(&format!("lists.{}", list), empty_list).unwrap();
            }
            Settings::write();
        }
    }

    /// Deletes an empty list, or deletes a crate from a list
    // TODO: allow deleting multiple crates
    pub fn del(list: &str, crat: Option<&str>) {
        if Self::exists(list) {
            {
                let mut settings = SETTINGS.write().unwrap();

                //println!("{:?}", settings.cache); // DEBUG
                //return;

                if let Ok(mut lists) = settings.get_table("lists") {
                    // TODO: delete the crate from the list
                    if let Some(c) = crat {

                    // delete the list, if empty
                    } else {
                        if lists[list].to_owned().into_array().unwrap().is_empty() {
                            println!("Deleting the empty list \"{}\".", list.bright_green());
                            //let _ = lists.remove(list);

                            // https://github.com/mehcode/config-rs/issues/108
                            // doesn't work :/
                            //let _ = settings.del("lists");
                            //let _ = settings.del(&format!("lists.{}", list));

                            // // TODO: reset the existing lists, except this one
                            // doesn't work either :/
                            // let new_lists: HashMap<String, Vec<String>> = HashMap::new();
                            // //let _ = settings.set("lists", true);
                            // let _a = settings.set("lists.favorites2", "sd");
                            // println!("\n»» {:?}", _a); // DEBUG

                            // if let Some(all_lists) = Self::get_lists(&settings) { 
                            //     for (key, value) in all_lists.iter() {
                            //         println!("{:?} {:?}", key, value);
                            //         //settings.set(&format!("lists.{}", key), vec!["as", "es"]).unwrap();
                            //     }
                            // }

                            // TODO: write the settings back
                            //settings.set("lists", lists).unwrap(); // TESTING
                            //settings.set("lists", vec!["pepe = [\"crin\", \"cran\"]"]).unwrap();
                            //settings.merge(lists).unwrap(); // TESTING

                            println!("\n>>{:?}", settings.get_table("lists")); // DEBUG


                        } else {
                            println!("The list \"{}\" cannot be deleted because it's not empty.",
                                list.bright_red());
                        }
                    }
                }
            }
            Settings::write(); // TODO: only write if changes happened
        } else {
            println!("List \"{}\" doesn't exist.", list.red());
        }
    }
    /// Get all the saved lists as a HashMap
    pub fn get_lists(settings: &Config) -> Option<HashMap<String, Vec<String>>> {

        let mut map = HashMap::new();

        if let Ok(lists) = settings.get_table("lists") {
            if !lists.is_empty() {
                // TODO generate map
                map.insert("lista1".to_string(), vec!["crate1".to_string(), "crate".to_string()]);
                return Some(map);

                // let mut lists_str = "".to_string();
                // for (list, _) in lists {
                //     lists_str = format!("{}, {} {}",
                //         lists_str, list.bright_green(),
                //         format!("({})", Self::quantity(&list)).cyan()
                //     );
                // }
                // println!("Your lists:\n{}", lists_str[1..].trim());
            }
        }
        None
    }


    /// Show the saved lists
    pub fn show_lists(recursive: bool) {
        let settings = SETTINGS.read().unwrap();
        let msg_empty = format!("You have no lists. Create a new one with '{}'",
            "crin list new <listname>".bright_blue());

        if let Ok(lists) = settings.get_table("lists") {
            if lists.is_empty() {
                println!("{}", msg_empty);
            } else {
                if recursive {
                    // for (list_name, value) in lists {
                    //         if let Ok(array) = value.into_array() {
                    //             println!("{}", list_name);
                    //             for crate_name in array {
                    //                 println!("\t{}", crate_name);
                    //             }
                    //         }
                    // }
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
                    println!("Your lists:\n{}", lists_str[1..].trim());
                }
            }
        } else {
            println!("{}", msg_empty);
        }

    }
}

