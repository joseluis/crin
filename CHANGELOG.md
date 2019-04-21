# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### feature: config

- read a configuration file from the proper path depending on the OS.
- manage lists of crates. Implement the following subcommands:
  - list       # show the lists, with the number of crates it contains
  - list show  # same as previous
  - list show <list>    # shows the crates contained in a list
  - list show <list> -i # shows the information for each crate
  - list new <list>     # creates a new list
  - list add <list> <crate> # adds the crate to the list
- write settings back to the configuration file.
  - save header, lists.


## 0.1.0 - 2019-04-15
First release. You can:

- Show crate information.
- Optionally show reverse dependencies as a number (`-r`) or a list (`-rr`)
- Search crates by query. Show the first 100 matches
- Show a global summary.
- Show specific summary for new crates.

