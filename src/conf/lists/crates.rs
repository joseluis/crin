use std::fmt;
use std::ops::{Deref, DerefMut};

use colored::*;

/// Represents a crate with its associated data
#[derive(Debug)]
pub struct Crate {
    name: String,
    note: String,
    // version: String,
    // tags: Vec<String>
}

impl Crate {
    pub fn new(name: &str, note: &str) -> Crate {
        Crate{
            name: name.to_string(),
            note: note.to_string()
        }
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       // TODO: add suffix "*" if it has a note
        write!(f, "{}", self.name
        )
    }
}


/// Represents a list of crates
#[derive(Debug)]
pub struct CrateList(Vec<Crate>);


impl CrateList {
    pub fn new() -> CrateList {
        CrateList(Vec::new())
    }
    pub fn push(&mut self, c: Crate) {
        self.0.push(c);
    }
}

impl fmt::Display for CrateList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "[".white())?;

        let mut first = true;
        for item in &self.0 {
            if !first {
                write!(f, "{} {}", ",".white() ,item)?;
            } else {
                write!(f, "{}", item)?;
            }
            first = false;
        }
        write!(f, "{}", "]".white())?;
        Ok(())
    }
}

impl IntoIterator for CrateList {
    type Item = Crate;
    type IntoIter = std::vec::IntoIter<Crate>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
