# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

- Big inner code refactoring to increase modularity and maintainability.
- Add subcommand aliases:`lists` for `list`, and `create` for `new`.
- Add `rem-all` command (alas `remove-all`)
- Replace generic unwrap() calls by expect(U) where U is a UUID String in base64,
  for easy line identification in the case of a panic.


## IN PROGRESS

- Support nested lists




## TODO

- Add `clone` subcommand, for cloning the git repository.
- Add `conf` subcommand.
  - show the configuration file path.
- » function to check if a crate is not valid, and say "it's not a valid crate" when checking
- LISTS:
  - Manage sublists
    - syntax? crin list move
    - move lists out and inside...
  - Ask for confirmation for removing-all and \*-all operations (or use -y to avoid to confirm)
  - add command for list, sub (substitute) crate (remove one, add another one)
  - add command for list, reordering (alphabetic)
    - add command for list, reorder crate (move to first, last, or to N position
      - reorder by number of downloads, updated, etc
        - remember ordering, and show in the header...
  - change the inner format for a list to...
    ordering = "alpha" # custom …
    crates =  [ { x = 1, y = 2, z = 3 },
           { x = 7, y = 8, z = 9 },
           { x = 2, y = 4, z = 8 } ]

    - DOUBTS: (de)serialize struct or an enum? examples:
      - https://github.com/alexcrichton/toml-rs/issues/265
      - https://github.com/alexcrichton/toml-rs/issues/286

- Add flag for no colors
- make arg flags to Override settings
  - but don't update the config file... (store in other place?)
    - unless using an argument for that: --save

- to determine if home == repo, ignore .git suffix

## 0.2.0 - 2019-04-21
Config file & list management update.

- Read a TOML configuration file from the proper path depending on the OS.
- Manage lists of crates. Implement the following subcommands:
  - `list`          # shows the lists
  - `list show`     # shows the lists
  - `list show -i`  # shows the lists and the crates they contain
  - `list show <list>`    # shows the crates contained in a list
  - `list show <list> -i` # shows the information for each crate in the list
  - `list new <list>`     # creates a new list
  - `list del <list>`     # deletes a list (alias `delete`)
  - `list add <list> <crate>` # adds the crate to the list
  - `list rem <list> <crate>` # removes the crate from the list (alias `remove`)
- Write settings back to the configuration file.
  - Save the lists after each modification.
- Change `crate` subcommand to `show` (left `crate` and `info` as an aliases)

## 0.1.0 - 2019-04-15
First release. You can:

- Show crate information.
- Optionally show reverse dependencies as a number (`-r`) or a list (`-rr`)
- Search crates by query. Show the first 100 matches
- Show a global summary.
- Show specific summary for new crates.

