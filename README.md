# crin

(**cr**ate **in**formation)

A handy CLI for the [crates.io](https://crates.io) API

<img src="https://github.com/joseluis/crin/blob/master/res/screenshot.png?raw=true" width="860"  height="630"/>

## Features

- Colorful, condensed & parseable output
- Show crate information
- Search for crates
- Global summary

### Planned features

- improved search
  - filter by keyword & category
  - sort alphabetically or by downloads
  - choose the page and results per page
- configuration file
  - favourites list & custom lists
  - customize colors
- show dependencies
- show version list
  - download any version

## Usage examples

### Crate Information
```sh
$ crin crate regex-syntax

# If you want to show the number of reverse dependencies  use `-r` or `--reverse`:
$ crin crate -r regex-syntax

# If you want the full list of reverse dependencies use `-rr` or `--reverse --reverse`:
$ crin crate -rr regex-syntax
```

### Search
```sh
$ crin search network
```


### Summary

```sh
$ crin summary

# More details on new crates
$ crin summary new
```

### Help
```
$ crin help
$ crin help crate
$ crin help summary new
```

## Installation

```sh
$ cargo install crin
```
