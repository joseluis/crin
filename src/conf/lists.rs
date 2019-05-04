use std::fmt;

use crate::conf::*;
use toml_edit::{value, Value, Array, Table};
use crate::util::commify;

/// Represents a crate with its associated data
#[derive(Debug)]
pub struct Crate {
    name: String,
    note: String,
    // version: String,
    // tags: Vec<String>
}

/// Represents a list of crates
#[derive(Debug)]
pub struct CrateList(Vec<Crate>);

impl CrateList {
    pub fn new(crates: Vec<Crate>) -> CrateList {
        CrateList(crates)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       // TODO: add suffix "*" if it has a note
        write!(f, "{}", self.name
        )
    }
}

impl fmt::Display for CrateList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for item in &self.0 {
            if !first {
                write!(f, ", {}", item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        write!(f, "]")?;
        Ok(())
    }
}


pub const ROOT     : bool = true;
pub const NOT_ROOT : bool = false;
pub const RECURSIVE     : bool = true;
pub const NOT_RECURSIVE : bool = false;
pub const PRINT_CRATES      : bool = true;
pub const DONT_PRINT_CRATES : bool = false;


/// Container of methods to manage lists in the config file
pub struct Lists {}

impl Lists {

    /// Checks whether a list exists
    ///
    /// A list name can be separated by dots, which indicates nesting. E.g.:
    /// The list name: "graphics.3d" would correspond to the TOML table:
    /// [lists.graphics.3d]
    ///
    pub fn exists(list: &str) -> bool {
        let settings = SETTINGS.read().expect("Error: Couldn't read the settings.");

        // check the root table [lists] exists
        if settings.as_table().contains_table("lists") {

            // begin the search there
            let mut current_list = settings["lists"].as_table().expect("ciDFsyL1SZ2Z_T9sghOtAA");

            // check all the nesting levels
            for l in list.split('.') {
                if current_list.contains_table(l) {
                    current_list = current_list[l].as_table().expect("JX9rg9rfTYm8Vo7bLy9o1w");
                } else {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    /// Checks whether an existing list contains crates
    fn has_crates(table: &Table) -> bool {
        if table.contains_key("crates") {
            if let Some(array) = table["crates"].as_array() {
                if !array.is_empty() {
                    return true;
                }
            }
        }
        false
    }

    /// Checks whether an existing list contains a description
    fn has_description(table: &Table) -> bool {
        if table.contains_key("description") {
            if let Some(text) = table["crates"].as_str() {
                if text.is_empty() {
                    return true;
                }
            }
        }
        false
    }

    /// Checks whether an existing list is empty. Meaning it doesn't contain any:
    /// - crates
    /// - description
    fn is_empty(table: &Table) -> bool {
        if Self::has_crates(table)
        || Self::has_description(table) {
            return false;
        }
        true
    }


    /// If a list exists, returns the Table
    ///
    // TODO: make a version that doesn't clone?
    pub fn as_table(list_name: &str) -> Option<Table> {
        let settings = SETTINGS.read().expect("fOjXn-7eQgWFTO53e5rkQg");

        // check the root table [lists] exists
        if settings.as_table().contains_table("lists") {
            let mut current_list = settings["lists"].as_table().expect("6vRmyJpkSmKa03Qdz-R_tg");

            if list_name == "" {
                return Some(current_list.to_owned());
            }

            // check all the nesting levels of the list_name
            for l in list_name.split('.') {
                if current_list.contains_table(l) {
                    current_list = current_list[l].as_table().expect("0mh2iuqXRVu3sGapafjDPw");
                } else {
                    return None;
                }
            }
            return Some(current_list.to_owned());
        }
        None
    }


    /// If a list exists, returns the Table as mutable
    ///
    pub fn as_table_mut(list_name: &str) -> Option<Table> {
        let mut settings = SETTINGS.write().expect("F7f4vwfvSWidMcNXY1Sltw");

        // check the root table [lists] exists
        if settings.as_table().contains_table("lists") {
            let mut current_list = settings["lists"].as_table_mut().expect("60ufriz1RIukmihae_rAmw");

            if list_name == "" {
                return Some(current_list.to_owned());
            }

            // check all the nesting levels of the list_name
            for l in list_name.split('.') {
                if current_list.contains_table(l) {
                        current_list = current_list[l].as_table_mut().expect("rM5UW3PERHqIqugA5gtRLw");
                } else {
                    return None;
                }
            }
            return Some(current_list.to_owned());
        }
        None
    }


    // TODO: join together the show() and show_lists function
    //
    // Both must support:
    // - showing crates
    // - showing sublists
    // - recursive
    // - different formatting options
    //
    // Differences (for now):
    // - show:
    //   - returns (optionally) a String
    // - show_lists:
    //   - doesn't return anything
    //   - prints
    //
    // Use cases of the new function:
    // - prepare a list of crates
    //   - for parsing (plain text separated with spaces)
    //   - for presentation (colorful and separated by commas)
    //   - optional recursion (crates of descendants)
    // - prepare a list of lists
    //   - either provide a list or return all in the root
    //   - optional recursion (all the nested lists)
    //   - for parsing
    //   - for presentation (newlines, tree structure)
    // - prepare a list of lists and crates
    //   - for presentation (newlines, tree structure)
    //      - this may call this function recursively on each nesting level
    // - choose to return or to print (return None)
    //

    // The root table [list] can't have crates, only its descendants.

    /// Prints the list names, as a tree
    ///
    /// Parameters
    /// ======================
    /// list:          &str   will print its children as Lists.
    /// recursive:      bool  true  => will show all the nested lists under «table».
    ///                       false => will show only the immediate children of «table».
    /// print_crates:   bool  true  => will print the crates of each shown list, at EOL.
    ///                       false => wont print the crate names.
    ///
 
    /// - Optional recursivity
    /// - Optionally prints the crates
    /// - If no list name is provided ("") starts from the root.
    ///
    pub fn print(list: &str, recursive: bool, print_crates: bool) {
        let settings = SETTINGS.read().expect("9xWCNf0KQv-3GLqnhbuiKg");
        let mut current_list;

        if list == "" {
            if let Some(list) = settings["lists"].as_table() {
                Self::print_children(list, "", recursive, print_crates);
            }
        } else {
            if let Some(ref ltable) = Self::as_table(list) {
                current_list = ltable;
                Self::print_children(current_list, "", recursive, print_crates);
            }
        }
    }

    /// Prints all the lists in a single nesting level
    /// [private]
    ///
    /// Parameters
    /// ======================
    /// table:         &Table  will print its children as Lists.
    /// indent:        &str    starting indentation level list_name prefix.
    /// recursive:      bool   true  => will show all the nested lists under «table».
    ///                        false => will show only the immediate children of «table».
    /// print_crates:   bool   true  => will print the crates of each shown list, at EOL.
    ///                        false => wont print the crate names.
    ///
    ///
    /// Notes
    /// ---------------------
    /// - The parent list «table» is never shown
    ///
    ///
    /// Output format
    /// ---------------------
    /// recursive && !print_crates =>
    ///     "indent"list_name(num_crates)
    ///
    /// recursive && print_crates =>
    ///     "indent"list_name(num_crates): [crates...]
    ///
    /// !recursive && !print_crates =>
    ///     "indent"list_name(num_crates) +[total_nested_lists(num_crates)]
    ///
    /// !recursive && print_crates =>
    ///     "indent"list_name(num_crates) +[total_nested_lists(num_crates)]: [crates...]
    ///
    fn print_children(table: &Table, indent: &str, recursive: bool, print_crates: bool) {
        for (list_name, _value) in table.iter() {
            if table.contains_table(list_name) {
                let mut text = format!("{}{}", indent, list_name.bright_green());

                let table = table[list_name].as_table().expect("vrRPW5IHTPOz6wwxKAERUw");
                let crates_num = Self::crates_num(table, NOT_RECURSIVE, NOT_ROOT);

                // The number of crates
                text = format!("{} {}", text, format!("({})",crates_num).green());

                let mut rtext = "".to_string();

                if !recursive {
                    // The number of children the list has
                    let table_children = Self::children_num(table, RECURSIVE);
                    if table_children > 0 {
                        rtext = format!("{}{}", " +[".cyan(),
                            format!("{}", table_children).bright_cyan());
                    }

                    // The number of crates in the list
                    let crates_num = Self::crates_num(table, RECURSIVE, NOT_ROOT) - crates_num;
                    if crates_num > 0 {
                        rtext = format!("{}{}", rtext, format!("({})]", crates_num).cyan());
                    }
                }
                text = format!("{}{}", text, rtext);

                if print_crates && crates_num > 0 {
                    let crates = Self::crates(table, NOT_RECURSIVE);
                    text = format!("{}: {}", text, crates).green().to_string();
                }
                println!("{}", text);

                if recursive {
                    Self::print_children(table, &format!("{}{}", indent, "·"), RECURSIVE, print_crates);
                }
            }
        }
    }


    /// Returns the number of lists inside a list
    ///
    /// Parameters
    /// ======================
    /// list:          &Table  will print its children as Lists.
    /// recursive:      bool   true  => count all the nested lists under «table».
    ///                        false => count only the immediate children of «table».
    ///
    /// Notes
    /// ---------------------
    /// - The parent list «table» is never counted
    ///
    pub fn children_num(list: &Table, recursive: bool) -> usize {
        let mut count = 0_usize;

        for (child, _) in list.iter() {
            if let Some(table) = list[child].as_table() {
                count +=1;
                if recursive {
                    count += Self::children_num(table, RECURSIVE);
                }
            }

        }
        count
    }

    /// Returns the number of crates in a list
    ///
    /// Optionally recursive.
    /// It always counts the number of crates in the provided (parent) list.
    ///
    /// list:     &Table
    /// recursive: bool  indicates
    /// is_root:   bool  ignore «list» and instead start from the root (the Table [lists])
    ///
    pub fn crates_num(list: &Table, recursive: bool, is_root: bool) -> usize {
        let mut count = 0_usize;

        // ignore a possible "crates" key child of [lists], since that Table is not a "real" list.
        //
        // NOTE: if «list» was a String, an empty String "" could mean the root table. But
        //       receiving a &Table doesn't make possible to determine if that table was the root.
        //       IDEA1: Unless there's a special key that *should* only be under the root.
        //       IDEA2: Unless we compare both tables for equality
        //       
        if !is_root {
            if list.contains_key("crates") {
                if let Some(array) = list["crates"].as_array() {
                    count += array.len();
                }
            }
        }

        if recursive {
            for (child, _) in list.iter() {
                if let Some(table) = list[child].as_table() {
                    count += Self::crates_num(table, RECURSIVE, NOT_ROOT);
                }
            }
        }
        count
    }

    /// Returns the crates in a list
    ///
    /// Optionally recursive.
    ///
    // TODO: make it work
    // TODO: implement recursivity
    pub fn crates(list: &Table, recursive: bool) -> CrateList {
        let mut crates_list = CrateList::new(vec![]);

        if Self::has_crates(list) {
            crates_list

        }
        crates_list
    }


    /// Shows the crates contained in a list,
    /// either as plain text, or colored & separated by commas
    // FIXME: implement functionality with new system, nested lists…
    // TODO: make it work
    pub fn show(list: &str, plain: bool) -> Option<String> {

        if Self::exists(list) {
            let settings = SETTINGS.read().expect("FG8CGac8RuWyi6_VuJpPwQ");



            /* OLD

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
            */
        } else {
            println!("List \"{}\" doesn't exist.", list.red());
        }
        None
    }


/*
    /// Adds or replaces a Note from a Crate
    ///
    // TODO: implement functionality
    // TODO: allow replace existing note with --force (-f)
    pub fn note(list:&str, note:&str) {

        if Self::exists(list) {

            //if

            //settings["lists"][list]["note"] = value(note);
        } else {
            println!("List \"{0}\" doesn't exist. You can create it with '{1}'",
                list.bright_red(), format!("crin list new {}", list.bright_green()).bright_blue());
        }
    }
*/

    /// Adds one crate to a list
    // FIXME: support nested lists, and the new crates format
    //   - check the key "crates" exists
    //   -> list["crates] = value(Array::default());
    // TODO: allow adding multiple crates
    pub fn add(list: &str, crat: &str) {

        if Self::exists(list) {

            if Self::has_crates(&Lists::as_table(list).expect("HYzjuOPxQNuegtF3W6YUDw")) {



            } else {

                // TODO: CREATE new
            }

            /* OLD

            //let mut crates_vec: Vec<&str>;

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
                    let mut settings = SETTINGS.write().expect("O2QAGCqZT-SozY7272yg6Q");
                    settings["lists"][list] = value(crates_arr);
                    }
                    println!("Added crate \"{}\" to the list \"{}\"",
                         crat.green(), list.bright_green());

                    Settings::write();
                }
            } else {
                println!("The list \"{}\" is empty.", list.red());
            }
            */

        } else {
            println!("List \"{0}\" doesn't exist. You can create it with '{1}'",
                list.bright_red(), format!("crin list new {}", list.bright_green()).bright_blue());
        }
    }

    /// Creates a new list
    ///
    // TODO: allow force create (to delete without emptying)
    pub fn new(list: &str, force: bool) {
        if Self::exists(list) && !force {
            println!("List \"{}\" already exists.", list.red());
        } else {
            {
                let mut settings = SETTINGS.write().expect("zEBZuweeRK6kBWoO5M1fqg");

                // first make sure the table [lists] exists
                if !settings.as_table().contains_table("lists") {
                    settings.as_table_mut()["lists"] = toml_edit::Item::Table(<Table>::new());
                }
                settings["lists"][list] = toml_edit::table();
            }
            Settings::write();
        }
    }

    /// Deletes an empty list
    ///
    // TODO: allow recursive (which deletes all empty descendant lists)
    // TODO: allow deleting multiple lists
    pub fn del(list: &str) {

        if Self::exists(list) {
            let mut changed = false;

            if Self::has_crates(&Lists::as_table(list).expect("oOGq_on-Q8CPMvEByFEUug")) {
                 println!("The list \"{}\" cannot be deleted because it's not empty.",
                    list.bright_red());
            } else {
                let mut settings = SETTINGS.write().expect("U1-isI-vR9OGSj4wxzVN6A");

                if let Some(table) = settings["lists"].as_table_mut() {
                    println!("Deleting the empty list \"{}\".", list.bright_green());
                    table.remove(list);
                    changed = true;
                } else {
                    println!("Error: couldn't delete the list \"{}\".", list.red());
                }
            }

            if changed { Settings::write(); }

        } else {
            println!("The list \"{}\" doesn't exist.", list.red());
        }


/*
        if Self::exists(list) {
            let mut changed = false;
            {
                let mut settings = SETTINGS.write().expect("6xWYx4kiSVODVjyKp-8-8w");

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
                    println!("With the contents: {}", settings["lists"][list].as_value().expect("6xWYx4kiSVODVjyKp-8-8w"));
                    // TODO: will delete if provided with force argument
                }

            }
            if changed { Settings::write(); }
        } else {
            println!("List \"{}\" doesn't exist.", list.red());
        }
    }
*/
    }

    /// Removes a crate from a list
    // FIXME: support nested lists, and new crates format
    // TODO: allow deleting multiple crates, maybe receiving a clap::Values struct
    pub fn rem(list: &str, crat: &str) {

        if Self::exists(list) {


            /* old
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
                    let mut settings = SETTINGS.write().expect("W_Kng1i2R9yPykSoNrhdQA");
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
            */
        } else {
            println!("List \"{}\" doesn't exist.", list.red());
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
