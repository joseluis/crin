# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### feature: config

- read a configuration file from the proper path depending on the OS.
- manage lists of crates. Implement the following subcommands:
  - list          # shows the lists
  - list show     # shows the lists
  - list show -i  # shows the lists and the crates they contain
  - list show <list>    # shows the crates contained in a list
  - list show <list> -i # shows the information for each crate in the list
  - list new <list>     # creates a new list
  - list del <list>     # deletes a list
  - list add <list> <crate> # adds the crate to the list
  - list rem <list> <crate> # removes the crate from the list
- write settings back to the configuration file.
  - save the lists after each modification.


## 0.1.0 - 2019-04-15
First release. You can:

- Show crate information.
- Optionally show reverse dependencies as a number (`-r`) or a list (`-rr`)
- Search crates by query. Show the first 100 matches
- Show a global summary.
- Show specific summary for new crates.

