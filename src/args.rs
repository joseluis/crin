use super::{VERSION, APPNAME};

use clap::{ArgMatches, Arg, App, AppSettings, SubCommand};
use colored::*;

use crate::actions::Actuator;
use crate::conf::lists::{Lists,
    ROOT, NOT_ROOT, RECURSIVE, NOT_RECURSIVE, PRINT_CRATES, DONT_PRINT_CRATES};

pub struct CliArguments<'a> {
    matches: ArgMatches<'a>
}

impl<'a> CliArguments<'a> {

    pub fn new() -> CliArguments<'a> {
        CliArguments {
            matches: Self::set_args()
        }
    }

    /// Defines the CLI arguments
    fn set_args() -> ArgMatches<'a> {

        // crin
        App::new(APPNAME)
            .version(VERSION)
            .setting(AppSettings::VersionlessSubcommands)
            .setting(AppSettings::UnifiedHelpMessage)

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

            // crin summary
            .subcommand(SubCommand::with_name("summary")
                .about("Show a summary")
                .setting(AppSettings::UnifiedHelpMessage)

                // crin summary new
                .subcommand(SubCommand::with_name("new")
                    .about("Detailed summary of the new crates")
                )
            )

            // crin show
            .subcommand(SubCommand::with_name("show")
                .about("Show crate info")
                 .visible_aliases(&["crate", "info"])

                 // crin show <crate>
                .arg(Arg::with_name("crate_name")
                    .help("the name of the crate")
                    //.index(1)
                    .required(true)
                )

                // crin show  <crate> [-r… ]
                .arg(Arg::with_name("reverse")
                     .short("r")
                     .long("reverse")
                     .help("show reverse dependencies (multi)")
                     .required(false)
                     .multiple(true)
                )
            )

            // crin search
            .subcommand(SubCommand::with_name("search")
                .about("Search for crates. Shows 1 page of 100 results")

                // crin search [query]
                .arg(Arg::with_name("query")
                    .help("the search query")
                    //.index(1)
                    .required(false)
                    .empty_values(false)
                )
                /*
                // crin search […] [-c [category]]
                .arg(Arg::with_name("category")
                     .short("c")
                     .help("limit query by category")
                     .required(false)
                     .empty_values(true)
                )
                // crin search […] [-k [keyword]]
                .arg(Arg::with_name("keyword")
                     .short("k")
                     .help("limit query by keyword")
                     .required(false)
                     .empty_values(true)
                )
                // crin search […] [-u [user]]
                .arg(Arg::with_name("user")
                     .short("u")
                     .help("limit query by user_id")
                     .required(false)
                     .empty_values(true)
                )
                */
            )

            // crin conf
            .subcommand(SubCommand::with_name("conf")
                .visible_alias("config")
                .about("manage the configuration")
            )

            // crin list
            .subcommand(SubCommand::with_name("list")
                .about("Manage your lists of crates")
                .visible_alias("lists")

                // crin list show
                .subcommand(SubCommand::with_name("show")
                    .about("shows the crates contained in the list")
                    // crin list show [list]
                    .arg(Arg::with_name("list")
                         .required(false)
                         .empty_values(false)
                    )
                    // crin list show [-i… ]
                    .arg(Arg::with_name("info")
                         .short("i")
                         .long("info")
                         .help("show the information of each crate in the list")
                         .required(false)
                         .multiple(true)
                         .empty_values(true)
                     )
                )

                // crin list desc
                .subcommand(SubCommand::with_name("desc")
                    .about("add a description to a list")
                    .visible_alias("description")

                    // crin list add <list>
                    .arg(Arg::with_name("list")
                         .help("the list where to add the description")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    // crin list add <list> <description>
                    .arg(Arg::with_name("description")
                         .help("the list description")
                         .required(true)
                         .empty_values(false)
                         .index(2)
                    )
                    // crin list add <list> <description> [-f ]
                    .arg(Arg::with_name("force")
                         .short("f")
                         .long("force")
                         .help("forces replacing the description")
                         .required(false)
                         .multiple(false)
                         .empty_values(true)
                     )
                )



                // crin list add
                .subcommand(SubCommand::with_name("add")
                    .about("add a crate to a list")
                    // crin list add <list>
                    .arg(Arg::with_name("list")
                         .help("the list where to add the crate")
                         .required(true)
                         .empty_values(false)
                         .index(1)
                    )
                    // crin list add <list> <crate>
                    .arg(Arg::with_name("crate")
                         .help("the crate to add to the list")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                         .index(2)
                    )
                )

                // crin list new
                .subcommand(SubCommand::with_name("new")
                    .visible_alias("create")
                    .about("create a new empty list")
                    // crin list new <list>
                    .arg(Arg::with_name("list")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                    )
                )

                // crin list del
                .subcommand(SubCommand::with_name("del")
                    .about("delete an empty list")
                    .visible_alias("delete")
                    // crin list del <list>
                    .arg(Arg::with_name("list")
                         .help("The list to delete (must be empty)")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                    )
                )

                // crin list rem
                .subcommand(SubCommand::with_name("rem")
                    .about("remove a crate from a list")
                    .visible_alias("remove")
                    // crin list rem <list>
                    .arg(Arg::with_name("list")
                         .help("The list containing the crate")
                         .required(true)
                         .empty_values(false)
                    )
                    // crin list rem <list> <crate>
                    .arg(Arg::with_name("crate")
                         .help("the crate to remove")
                         .required(true)
                         .empty_values(false)
                         //.multiple(true) // TODO: allow multiple
                    )
                )
                /* TODO:
                // crin list rem-all
                .subcommand(SubCommand::with_name("rem-all")
                    .about("remove all crates from a list")
                    .visible_alias("remove-all")
                    // crin list rem-all <list>
                    .arg(Arg::with_name("list")
                         .help("The list containing the crate")
                         .required(true)
                         .empty_values(false)
                    )
                )
                */

                /*
                // crin list copy
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

                // crin list move
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

                // crin list copy-all
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

                // crin list move-all
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

                // crin list clone
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
            )

            .get_matches()
        }


    /// Parses the received CLI arguments and triggers the appropriate actions
    pub fn parse(&self) {

        let act = Actuator::new();

        match self.matches.subcommand() {

            // crin show
            ("show", Some(crate_name)) => {
                let _ = act.show_crate(
                    crate_name.value_of("crate_name").expect("TBd5796SSnuZ9dG_j159sQ"),
                    crate_name.occurrences_of("reverse"));
            },

            // crin search
            //
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

            // crin summary
            //
            // TODO: add alias "sum"
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

            // crin list
            //
            // TODO: add list summary (alias: sum)
            ("list", Some(list_matches)) => {
                match &list_matches.subcommand() {

                    // crin list show
                    ("show", Some(args)) => {
                        if let Some(list) = args.value_of("list") {

                            if Lists::exists(list) {
                                println!("Your list \"{}\" contains {} crates:",
                                    list.bright_green(),
                                    Lists::crates_num(
                                        &Lists::as_table(list).expect("tGbnh-1XRmmYkD6b3V-9Cw"),
                                            NOT_RECURSIVE, NOT_ROOT)
                                    );

                                match args.occurrences_of("info") {
                                    0 => if let Some(contents) = Lists::crates(list, false) {
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
                                0 => {
                                    println!("You have {} lists containing {} crates:\n", 
                                        Lists::children_num(&Lists::as_table("").expect("c55xuxvtSo-cqkny-yD2Ew"),
                                            RECURSIVE),
                                        Lists::crates_num(&Lists::as_table("").expect("uuXMmCTpQsmWngZnNdiupA"),
                                            RECURSIVE, ROOT),
                                        );
                                    Lists::print("", NOT_RECURSIVE, PRINT_CRATES);
                                    //Lists::print("", NOT_RECURSIVE, DONT_PRINT_CRATES);
                                }
                                1 | _ => {
                                    println!("You have {} lists containing {} crates:\n", 
                                        Lists::children_num(&Lists::as_table("").expect("7SDTLcxgS1Wy7X0dXsJKLA"),
                                            RECURSIVE),
                                        Lists::crates_num(&Lists::as_table("").expect("VJGW-VluQmmD20X0yrIw9w"),
                                            RECURSIVE, ROOT),
                                        );
                                    //Lists::print("", RECURSIVE, PRINT_CRATES);
                                    Lists::print("", RECURSIVE, DONT_PRINT_CRATES);
                                }
                            }
                        }
                    },

                    // crin list new
                    ("new", Some(args)) => {
                        if let Some(list) = args.value_of("list") {
                            println!("creating: {}", list);
                            Lists::new(list, false);
                        }
                    },

                    // crin list del
                    ("del", Some(args)) => {
                        // TODO: allow multiple
                        if let Some(list) = args.value_of("list") { Lists::del(list); }
                    },

                    // crin list add
                    ("add", Some(args)) => {
                        // TODO: allow multiple
                        Lists::add(args.value_of("list").expect("CBn1qxp1TMeMIYMMv7bj8w"),
                            args.value_of("crate").expect("1Jc8xPhiQyCYrFQz3LY92Q"));
                    },

                    // crin list rem
                    ("rem", Some(args)) => {
                        // TODO: allow multiple
                        Lists::rem(args.value_of("list").expect("kk4kT7stQDmSMxo1ibgTZg"),
                        args.value_of("crate").expect("yAS9OSIOQ3y0Bckx8SHULQ"));
                    },

                    /*
                    // TODO:
                    ("copy", Some(args)) => {
                        let list_from = args.value_of("list_from").expect("hdact_gDTRSTEXsbRym02w");
                        let list_to = args.value_of("list_to").expect("bWRYdAMzRtWWblEXPZTpqA");

                        if list_from == list_to {
                            println!("You must copy between different lists");
                        } else {
                            // TODO check both lists exist, and crate too
                            println!("copy: {} to {}", list_from, list_to);
                        }
                    },
                    // TODO:
                    ("move", Some(args)) => {
                        let list_from = args.value_of("list_from").expect("zlbaECPBQOi7i9cP3eu4ow");
                        let list_to = args.value_of("list_to").expect("iaKAL_TdQPWMXAvIeEs9kg");

                        if list_from == list_to {
                            println!("You must move between different lists");
                        } else {
                            println!("move: {} to {}", list_from, list_to);
                        }
                    }
                    // TODO:
                    ("move-all", Some(args)) => {
                        let list_from = args.value_of("list_from").expect("VGTFhKr4TDefuRhbQ_gWNQ");
                        let list_to = args.value_of("list_to").expect("XFkXkajTRB6_80ZiKYHj5g");

                        if list_from == list_to {
                            println!("You must move between different lists");
                        } else {
                            println!("move all crates from: {} to {}", list_from, list_to);
                        }
                    }
                    // TODO:
                    ("clone", Some(args)) => {
                        let list_existing = args.value_of("list_existing").expect("S5DK1QvMQ8WSKK6AaZ2J6g");
                        let list_new = args.value_of("list_new").expect("S5DK1QvMQ8WSKK6AaZ2J6g");

                        // TODO check the list exist and the other doesn't
                        println!("cloning from {} to {}", list_existing, list_new);
                    },
                    */

                    // crin list
                    ("", None) => {
                        println!("You have {} lists containing {} crates:\n", 
                            Lists::children_num(&Lists::as_table("").expect("BiQz5dZPTPOJ59ZUfLqwIQ"), RECURSIVE),
                            Lists::crates_num(&Lists::as_table("").expect("f23pdUbuQS61a2uQOHUCmw"), RECURSIVE, ROOT),
                            );
                        Lists::print("", NOT_RECURSIVE, PRINT_CRATES);
                        //Lists::print("", NOT_RECURSIVE, DONT_PRINT_CRATES);
                    }

                    _ => unreachable!(),
                }
            }

            // crin
            _ => println!("{} help", APPNAME),

        }

    }

}
