// #![allow(dead_code)]
// #![allow(unreachable_code)]
// #![allow(unused_imports)]
// #![allow(unused_assignments)]
// #![allow(unused_variables)]
// #![allow(unused_mut)]

#[macro_use] extern crate lazy_static;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const ORGANIZATION: &'static str = "joseluis";
const APPNAME: &'static str = env!("CARGO_PKG_NAME");
const CONFIGNAME: &'static str = "config.toml";

mod args;
mod conf;
mod actions;
mod util;

fn main() {
    conf::Settings::read();
    args::CliArguments::new().parse();
}
