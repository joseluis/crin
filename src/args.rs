use super::{VERSION, APPNAME};

use clap::{ArgMatches, Arg, App, AppSettings, SubCommand};
use colored::*;

use crate::conf::Lists;
use crate::actions::Actuator;

pub struct CliArguments<'a> {
    matches: ArgMatches<'a>
}

impl<'a> CliArguments<'a> {

    /// Constructor
    pub fn new() -> CliArguments<'a> {
        CliArguments {
            matches: Self::set_args()
        }
    }

    /// Defines the arguments
    fn set_args() -> ArgMatches<'a> {

        App::new(APPNAME)
            .version(VERSION)
            .setting(AppSettings::VersionlessSubcommands)
            .setting(AppSettings::UnifiedHelpMessage)

            // global flags
            // TODO: add flag for no colors
            /*
            .arg(Arg::with_name("page")
                .short("p")
                .long("page")
                .takes_value(true)
                .value_name("pagenum")
                .help("the page of results to show (default 1)")
                .required(false)
                .global(true)
            )
            */

            // SUBCOMMANDS

            // summary
            .subcommand(SubCommand::with_name("summary")
                .about("Show a summary")
                .setting(AppSettings::UnifiedHelpMessage)

                // new
                .subcommand(SubCommand::with_name("new")
                    .about("Detailed summary of the new crates")
                )
            )

            // crate
            .subcommand(SubCommand::with_name("show")
                .about("Show crate info")
                 .visible_aliases(&["crate", "info"])

                .arg(Arg::with_name("crate_name")
                    .help("the name of the crate")
                    //.index(1)
                    .required(true)
                )
                .arg(Arg::with_name("reverse")
                     .short("r")
                     .long("reverse")
                     .help("show reverse dependencies (multi)")
                     .required(false)
                     .multiple(true)
                )
            )

            // search
            .subcommand(SubCommand::with_name("search")
                .about("Search for crates. Shows 1 page of 100 results")
                .arg(Arg::with_name("query")
                    .help("the search query")
                    //.index(1)
                    .required(false)
                    .empty_values(false)
                )
                /*
                .arg(Arg::with_name("category")
                     .short("c")
                     .help("limit query by category")
                     .required(false)
                     .empty_values(true)
                )
                .arg(Arg::with_name("keyword")
                     .short("k")
                     .help("limit query by keyword")
                     .required(false)
                     .empty_values(true)
                )
                .arg(Arg::with_name("user")
                     .short("u")
                     .help("limit query by user_id")
                     .required(false)
                     .empty_values(true)
                )
                */
            )


            .subcommand(SubCommand::with_name("list")
                .about("Manage your lists of crates")
                .visible_alias("lists")

                .subcommand(SubCommand::with_name("show")
                    .about("shows the crates contained in the list")
                    .arg(Arg::with_name("list")
                         .required(false)
                         .empty_values(false)
                    )
                    .arg(Arg::with_name("info")
                         .short("i")
                         .long("info")
                         .help("show the information of each crate in the list")
                         .required(false)
                         .multiple(true)
                         .empty_values(true)
                     )
                )
                .subcommand(SubCommand::with_name("add")
                    .about("add a crate to a list")
                    .arg(Arg::with_name("list")
                         .help("the list where to add the crate")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    .arg(Arg::with_name("crate")
                         .help("the crate to add to the list")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                         .index(2)
                    )
                )
                .subcommand(SubCommand::with_name("new")
                    .about("create a new empty list")
                    .arg(Arg::with_name("list")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                    )
                )
                .subcommand(SubCommand::with_name("del")
                    .about("delete an empty list")
                    .visible_alias("delete")

                    .arg(Arg::with_name("list")
                         .help("The list to delete (must be empty)")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                    )
                )
                .subcommand(SubCommand::with_name("rem")
                    .about("remove a crate from a list")
                    .visible_alias("remove")

                    .arg(Arg::with_name("list")
                         .help("The list containing the crate")
                         .required(true)
                         .empty_values(false)
                    )
                    .arg(Arg::with_name("crate")
                         .help("the crate to remove")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                    )
                )

                /*
                .subcommand(SubCommand::with_name("copy")
                    .help("copy a crate from one list to another")
                    .arg(Arg::with_name("list_from")
                         .help("the list and crate to copy from (list:crate)")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    .arg(Arg::with_name("list_to")
                         .help("the list where to copy the crate")
                         .required(true)
                         .empty_values(false)
                         .index(2)
                    )
                )
                .subcommand(SubCommand::with_name("move")
                    .help("move a crate from one list to another")
                    .arg(Arg::with_name("list_from")
                         .help("the list and crate to move from (list:crate)")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    .arg(Arg::with_name("list_to")
                         .help("the list where to move the crate")
                         .required(true)
                         .empty_values(false)
                         .index(2)
                    )
                )
                .subcommand(SubCommand::with_name("copy-all")
                    .help("copy all crates from one list to another")
                    .arg(Arg::with_name("list_from")
                         .help("the list from where all the crates will be copied")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    .arg(Arg::with_name("list_to")
                         .help("the list where to copy all the crates")
                         .required(true)
                         .empty_values(false)
                         .index(2)
                    )
                )
                .subcommand(SubCommand::with_name("move-all")
                    .help("move all crates from one list to another")
                    .arg(Arg::with_name("list_from")
                         .help("the list from where all the crates will be moved")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    .arg(Arg::with_name("list_to")
                         .help("the list where to move all the crates")
                         .required(true)
                         .empty_values(false)
                         .index(2)
                    )
                )
                .subcommand(SubCommand::with_name("clone")
                    .arg(Arg::with_name("list_existing")
                         .help("the list to clone")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    .arg(Arg::with_name("list_new")
                         .help("the new cloned list (must not exist)")
                         .required(true)
                         .empty_values(false)
                         .index(2)
                    )
                )
                */
                //
            )
            .get_matches()
        }


    /// Parses the CLI arguments
    pub fn parse(&self) {

        let act = Actuator::new();

        match self.matches.subcommand() {

            ("show", Some(crate_name)) => {
                let _ = act.show_crate(
                    crate_name.value_of("crate_name").unwrap(),
                    crate_name.occurrences_of("reverse"));
                },

            // TODO: improve usage of default values
            // TODO: allow passing page & page_num
            // TODO: add filters
            ("search", Some(query)) => {
                let _ = act.search_crate(query.value_of("query"),
                    if let Some(p) = self.matches.value_of("page") {
                        // If the number is not recognized, defaults to page 1
                        p.parse::<u64>().unwrap_or(1)
                        } else {1},
                    if let Some(pp) = self.matches.value_of("per_page") {
                        // If the number is not recognized, defaults to 100 per page
                        pp.parse::<u64>().unwrap_or(100)
                        } else {100}
                    );
                },

            ("summary", Some(summary_matches)) => {
                    match summary_matches.subcommand() {
                        ("new", Some(_)) => { let _ = act.show_summary_new_crates(); }
                        ("most_downloaded", Some(_)) => {}
                        ("most_recently_downloaded", Some(_)) => {}
                        ("recently_updated", Some(_)) => {}
                        ("popular_keywords", Some(_)) => {}
                        ("popular_categories", Some(_)) => {}
                        ("", None) => { let _ = act.show_summary(); },
                        _ => unreachable!()
                    }
                },

            // LIST ARGUMENTS

            ("list", Some(list_matches)) => {
                match &list_matches.subcommand() {
                    ("show", Some(args)) => {
                        if let Some(list) = args.value_of("list") {
                            if Lists::exists(list) {
                                println!("Your list \"{}\" contains {} crates:",
                                    list.bright_green(), Lists::quantity(list));

                                match args.occurrences_of("info") {
                                    0 => if let Some(contents) = Lists::show(list, false) {
                                        println!("{}", contents);
                                    }
                                    // TODO: move this to 2 or more occurences, and
                                    // make a more compact presentation for 1 occurrence
                                    1 | _ => if let Some(contents) = Lists::show(list, true) {
                                        for crate_name in contents.split_whitespace() {
                                            let _ = act.show_crate(crate_name,
                                                args.occurrences_of("reverse"));
                                            println!("");
                                        }
                                    }
                                }

                            } else {
                                println!("List \"{0}\" doesn't exist. You can create it with '{1}'",
                                    list.bright_red(), format!("crin list new {}",
                                        list.bright_green()).bright_blue());
                            }

                        } else {
                            // if no list is provided, show which lists there are
                            match args.occurrences_of("info") {
                                0 => Lists::show_lists(false),
                                1 | _ => Lists::show_lists(true),
                            }
                        }
                    },
                    ("new", Some(args)) => {
                        if let Some(list) = args.value_of("list") {
                            println!("creating: {}", list);
                            Lists::new(list);
                        }
                    },
                    ("del", Some(args)) => {
                        // TODO: allow multiple
                        if let Some(list) = args.value_of("list") { Lists::del(list); }
                    },
                    ("add", Some(args)) => {
                        // TODO: allow multiple
                        Lists::add(args.value_of("list").unwrap(), args.value_of("crate").unwrap());
                    },
                    ("rem", Some(args)) => {
                        // TODO: allow multiple
                        Lists::rem(args.value_of("list").unwrap(), args.value_of("crate").unwrap());
                    },
                    /*
                    // TODO:
                    ("copy", Some(args)) => {
                        let list_from = args.value_of("list_from").unwrap();
                        let list_to = args.value_of("list_to").unwrap();

                        if list_from == list_to {
                            println!("You must copy between different lists");
                        } else {
                            // TODO check both lists exist, and crate too
                            println!("copy: {} to {}", list_from, list_to);
                        }
                    },
                    // TODO:
                    ("move", Some(args)) => {
                        let list_from = args.value_of("list_from").unwrap();
                        let list_to = args.value_of("list_to").unwrap();

                        if list_from == list_to {
                            println!("You must move between different lists");
                        } else {
                            println!("move: {} to {}", list_from, list_to);
                        }
                    }
                    // TODO:
                    ("move-all", Some(args)) => {
                        let list_from = args.value_of("list_from").unwrap();
                        let list_to = args.value_of("list_to").unwrap();

                        if list_from == list_to {
                            println!("You must move between different lists");
                        } else {
                            println!("move all crates from: {} to {}", list_from, list_to);
                        }
                    }
                    // TODO:
                    ("clone", Some(args)) => {
                        let list_existing = args.value_of("list_existing").unwrap();
                        let list_new = args.value_of("list_new").unwrap();

                        // TODO check the list exist and the other doesn't
                        println!("cloning from {} to {}", list_existing, list_new);
                    },
                    */
                    ("", None) => { Lists::show_lists(false); }
                    _ => unreachable!(),
                }
            }

            _ => println!("{} help", APPNAME),

        }

    } // read()

}
