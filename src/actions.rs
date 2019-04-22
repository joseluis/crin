use crates_io_api::{SyncClient, Error, ListOptions, Sort, ReverseDependencies, Meta};
use num_format::{Locale, ToFormattedString};
use chrono::Local;
use colored::*;

use crate::util::*;

pub struct Actuator {
    client: SyncClient
}

impl Actuator {

    /// Constructor
    pub fn new() -> Actuator {
        Actuator {
            client: SyncClient::new()
        }
    }


    /// Show a short general summary
    pub fn show_summary(&self) -> Result<(), Error> {

        let summary = self.client.summary()?;

        println!("{}\t {}", "Total number of crates:".blue(),
            summary.num_crates.to_formatted_string(&Locale::en).bright_green());
        println!("{} {}", "Total number of downloads:".blue(),
            summary.num_downloads.to_formatted_string(&Locale::en).bright_red());

        { // popular keywords
            let mut counter = 0_usize;
            println!("\n{}\n{}", "Popular keywords:".bright_blue(),
                "# keyword crates_count created (ago)".bright_black());
            for c in summary.popular_keywords.iter() {
                counter += 1;
                println!("{} {} {} {} {}",
                    format!("{:2}", counter).bright_black(),
                    c.keyword.cyan(),
                    c.crates_cnt.to_formatted_string(&Locale::en).green(),
                    date_str(&c.created_at, "").purple(),
                    date_ago(&c.created_at, 1).purple(),
                );
            }
        }
        { // popular categories
            let mut counter = 0_usize;
            println!("\n{}\n{}", "Popular categories:".bright_blue(),
                "# category crates_count created (ago)".bright_black());
            for c in summary.popular_categories.iter() {
                counter += 1;
                println!("{} {} {} {} {}",
                    format!("{:2}", counter).bright_black(),
                    c.category.cyan(),
                    c.crates_cnt.to_formatted_string(&Locale::en).green(),
                    date_str(&c.created_at, "").purple(),
                    date_ago(&c.created_at, 1).purple(),
                );
            }

        }

        { // most downloaded
            let mut counter = 0_usize;
            println!("\n{}\n{}", "Most downloaded:".bright_blue(),
                "# crate_name max_version downloads created updated (ago) description".bright_black());
            for c in summary.most_downloaded.iter() {
                counter += 1;
                println!("{} {} {} {} {} {} {} {}",
                    format!("{:2}", counter).bright_black(),
                    c.name.green(),
                    c.max_version.yellow(),
                    c.downloads.to_formatted_string(&Locale::en).red(),
                    date_str(&c.created_at, "").purple(),
                    date_str(&c.updated_at, "").bright_purple(),
                    date_ago(&c.updated_at, 1).bright_purple(),
                    format!("{}", c.description.as_ref().unwrap().trim().replace('\n', ""))
                );
            }
        }
        { // most recently downloaded
            let mut counter = 0_usize;
            println!("\n{}\n{}", "Most recently downloaded:".bright_blue(),
                "# crate_name max_version downloads created updated (ago) description".bright_black());
            for c in summary.most_recently_downloaded.iter() {
                counter += 1;
                println!("{} {} {} {} {} {} {} {}",
                    format!("{:2}", counter).bright_black(),
                    c.name.green(),
                    c.max_version.yellow(),
                    c.downloads.to_formatted_string(&Locale::en).red(),
                    date_str(&c.created_at, "").purple(),
                    date_str(&c.updated_at, "").bright_purple(),
                    date_ago(&c.updated_at, 1).bright_purple(),
                    format!("{}", c.description.as_ref().unwrap().trim().replace('\n', ""))
                );
            }
        }
        { // just updated
            let mut counter = 0_usize;
            println!("\n{}\n{}", "Just updated:".bright_blue(),
                "# crate_name max_version downloads created updated (ago) description".bright_black());
            for c in summary.just_updated.iter() {
                counter += 1;
                println!("{} {} {} {} {} {} {} {}",
                    format!("{:2}", counter).bright_black(),
                    c.name.green(),
                    c.max_version.yellow(),
                    c.downloads.to_formatted_string(&Locale::en).red(),
                    date_str(&c.created_at, "").purple(),
                    date_str(&c.updated_at, "").bright_purple(),
                    date_ago(&c.updated_at, 1).bright_purple(),
                    format!("{}", c.description.as_ref().unwrap().trim().replace('\n', ""))
                );
            }
        }
        { // new crates
            let mut counter = 0_usize;
            println!("\n{}\n{}", "New crates:".bright_blue(),
                "# crate_name max_version downloads created updated (ago) description".bright_black());
            for c in summary.new_crates.iter() {
                counter += 1;
                println!("{} {} {} {} {} {} {} {}",
                    format!("{:2}", counter).bright_black(),
                    c.name.green(),
                    c.max_version.yellow(),
                    c.downloads.to_formatted_string(&Locale::en).red(),
                    date_str(&c.created_at, "").purple(),
                    date_str(&c.updated_at, "").bright_purple(),
                    date_ago(&c.updated_at, 1).bright_purple(),
                    format!("{}", c.description.as_ref().unwrap().trim().replace('\n', ""))
                );
            }
        }
        Ok(())
    }


    /// Show a more detailed summary of the new crates
    pub fn show_summary_new_crates(&self) -> Result<(), Error> {

        let summary = self.client.summary()?;

        for c in summary.new_crates {
            // name & version
            println!("{} {}", c.name.green(), c.max_version.yellow());
            // description
            if let Some(ref p) = c.description {
                println!("\t{}\t{}", "description:".blue(), p.trim().replace("\n", "\n\t\t\t"));
            }
            // created at
            println!("\t{}\t{} {}", "created:".blue(),
                date_str(&c.created_at, "%Y-%m-%d %H:%M").purple(),
                date_ago(&c.created_at, 1).bright_purple(),
                );
            // categories (always null)
            // if let Some(ref p) = c.categories { println!("{}\t{:?}", "categories:".blue(), p); }
            // keywords (always null)
            // if let Some(ref p) = c.keywords { println!("{}\t{:?}", "keywords:".blue(), p); }
            // repository
            if let Some(ref p) = c.repository { println!("\t{}\t{}", "repository:".blue(),
                p.bright_blue().underline()); }
            // homepage (if != repository)
            if let Some(ref p) = c.homepage {
                if let Some(ref p2) = c.repository {
                    if p != p2 {
                        println!("\t{}\t{}", "homepage:".blue(),  p.bright_blue().underline());
                    }
                }
            }
            // documentation
            if let Some(ref p) = c.documentation { println!("\t{}\t{}", "documentation:".blue(),
                p.bright_blue().underline()); }
            // downloads
            println!("\t{}\t{}", "downloads:".blue(),
                c.downloads.to_formatted_string(&Locale::en).red());
            // license
            if let Some(ref p) = c.license { println!("\t{}\t{}", "license:".blue(),
                p.bright_blue().italic()); }
            println!("");
        }
        Ok(())
    }


    /// Search for a crate.
    pub fn search_crate(&self, query: Option<&str>, page: u64, per_page: u64)
        -> Result<(), Error> {

        let res = self.client.crates(ListOptions{
            sort: Sort::Alphabetical,
            per_page: 100,
            page,
            query: if let Some(q) = query { Some(q.to_string())} else {None},
        })?;

        println!("{} \"{}\"", "Searching for: ", query.unwrap_or("\"*\""));

        if res.crates.len() > 0 {

            println!("Showing {} results of {} (page {} of {}) \n\n{}",
                {std::cmp::min(per_page, res.meta.total)}.to_string().bright_green(),
                res.meta.total.to_string().green(),
                page.to_string().bright_blue(),
                (res.meta.total / per_page + (res.meta.total % per_page != 0) as u64)
                    .to_string().bright_blue(),

                // TODO: add sorted by
                "# crate_name version created updated (ago) downloads (recent)"
                .bright_black());
        } else {
            println!("{}", "No results found.".red());
        }

        let mut counter = 0_usize;
        let mut counter_str;
        let counter_len = res.meta.total.to_string().len();

        for c in res.crates {
            counter += 1;
            counter_str = format!("{:width$}", counter, width = counter_len);

            let crate_name;
            match c.exact_match.as_ref() {
                Some(true) => crate_name = c.name.bright_green(),
                _ => crate_name = c.name.green(),
            }


            let recent_downloads;
            if let Some(dl) = c.recent_downloads {
                recent_downloads = format!("({})",
                    dl.to_formatted_string(&Locale::en)).bright_red();
            } else {
                recent_downloads = "(?)".bright_black();
            }

            // 1st line
            println!("{} {} {} {} {} {} {} {}",
                counter_str.bright_black(),
                crate_name,
                c.max_version.yellow(),
                date_str(&c.created_at, "").purple(),
                date_str(&c.updated_at, "").bright_purple(),
                date_ago(&c.updated_at, 1).bright_purple(),
                c.downloads.to_formatted_string(&Locale::en).red(),
                recent_downloads,
            );

            // 2nd line
            if let Some(desc) = c.description {
                let mut desc_tidy = String::new();

                for line in desc.lines() {
                    desc_tidy = format!("{} {}", desc_tidy, line.trim());
                }

                println!("{:width$}{}\n", " ".bright_black(),
                    desc_tidy.trim(),
                    width = counter_len + 1);
            }


        }
        Ok(())
    }


    /// Show crate information
    pub fn show_crate(&self, crate_name: &str, show_rdeps: u64)
        -> Result<(), Error> {

        let res = self.client.get_crate(crate_name)?;
        let c = res.crate_data;

        // name
        println!("{}\t\t{}", "name:".blue(), c.name.green());
        // version
        println!("{}\t{}", "version:".blue() , c.max_version.yellow());
        // description
        if let Some(ref p) = c.description {
            println!("{}\t{}", "description:".blue(), p.trim().replace("\n", "\n\t\t"));
        }
        // categories
        if res.categories.len() > 0 {
            let mut cat_str  = format!("{}", res.categories[0].category.cyan());
            for cat in res.categories.iter().skip(1) {
                cat_str = format!("{}, {}", cat_str, cat.category.cyan());
            }
            println!("{}\t{}", "categories:".blue(), cat_str);
        }
        // keywords
        if let Some(ref keywlist) = c.keywords {
            if keywlist.len() > 0 {
                let mut keyw_str  = format!("{}", keywlist[0].cyan());
                for keyw in keywlist.iter().skip(1) {
                    keyw_str = format!("{}, {}", keyw_str, keyw.cyan());
                }
                println!("{}\t{}", "keywords:".blue(), keyw_str);
            }
        }
        // created
        println!("{}\t{} {}", "created:".blue(),
            date_str(&c.created_at, "").purple(), date_ago(&c.created_at, 1).purple());

        // updated
        println!("{}\t{} {}", "updated:".blue(),
            date_str(&c.updated_at, "").bright_purple(), date_ago(&c.updated_at, 1).bright_purple());

        // downloads
        println!("{}\t{}", "downloads:".blue(),
            c.downloads.to_formatted_string(&Locale::en).red());
        // repository
        if let Some(ref p) = c.repository { println!("{}\t{}", "repository:".blue(),
            p.bright_blue().underline()); }
        // homepage (if != repository)
        if let Some(ref p) = c.homepage {
                if let Some(ref p2) = c.repository {
                    if p != p2 {
                        println!("{}\t{}", "homepage:".blue(),  p.bright_blue().underline());
                    }
                }
        }
        // documentation
        if let Some(ref p) = c.documentation { println!("{}\t{}", "documentation:".blue(), p.bright_blue().underline()); }
        // license
        if let Some(ref l) = res.versions[0].license { println!("{}\t{}",
            "license:".blue(), l.bright_blue().italic()); }
        // owners
        let owners = self.client.crate_owners(crate_name);
        if let Ok(ref olist) = owners {
            let mut ostr = "".to_string();
            for o in olist {
                if let Some(ref kind) = o.kind {
                    match kind.as_ref() {
                        "user" => ostr = format!("{}", ostr),
                        "team" => ostr = format!("{}team: ", ostr),
                        _ => () ,
                    }
                }
                if let Some(ref name) = o.name { ostr = format!("{}{}", ostr, name); }
                if let Some(ref email) = o.email { ostr = format!("<{}> {}", ostr, email); }
                ostr = format!("{} ({})\n\t\t", ostr, o.login);
            }
            println!("{}\t\t{}", "owners:".blue(), ostr.trim().yellow())
        }

        // reverse dependencies
        let mut revdep = ReverseDependencies {dependencies: Vec::new(), meta: Meta {total:0}};
        if show_rdeps > 0 {
            revdep = self.client.crate_reverse_dependencies(crate_name).unwrap();
            if revdep.meta.total > 0 {
                println!("{}\t{}", "reverse deps:".blue(),
                    revdep.meta.total.to_formatted_string(&Locale::en).bright_green());
            }
        }

        // show list of reverse dependencies
        if show_rdeps > 1 && revdep.meta.total > 0 {
            println!("{}" ,
                "# optional crate_name version requires created updated (ago) downloads bytes (size) license"
                .bright_black());

            let mut counter = 0;
            let mut counter_str;
            let counter_len = revdep.meta.total.to_string().len();

            for d in revdep.dependencies.iter() {
                let vreq = &d.dependency.req;
                let name = &d.crate_version.crate_name;
                let vnum = &d.crate_version.num;
                let downloads = d.crate_version.downloads;

                let mut size_bytes = "?".to_string().black();
                let mut size = "?".to_string().black();
                if let Some(s) = d.crate_version.crate_size {
                    size_bytes = s.to_string().blue();
                    size = format!("({})",
                        byte_unit::Byte::from_bytes(s as u128).get_appropriate_unit(true).to_string().replace(" ", "")).blue();
                }

                let mut license = "?".to_string().black();
                if let Some(l) = &d.crate_version.license {
                    license = l.bright_blue().italic();
                }

                let optional = d.dependency.optional;
                let optional_str;
                if optional {
                    optional_str = "O".bright_yellow();
                } else {
                    optional_str = ".".black();
                }

                let created = date_str(&d.crate_version.created_at, "");
                let updated = date_str(&d.crate_version.updated_at, "");
                //let updated = d.crate_version.updated_at.with_timezone(&Local);
                let updated_ago = Local::now().
                    signed_duration_since(d.crate_version.updated_at).to_std().unwrap();
                let updated_ago_str = format!("({})",
                    timeago::format_5chars(updated_ago).to_string());
                // let updated_ago_str = format!("({})", timeago::Formatter::new().num_items(1).abbreviate().ago("").convert(updated_ago).to_string());

                counter += 1;
                counter_str = format!("{:width$}", counter, width = counter_len);

                println!("{} {} {} {} {} {} {} {} {} {} {} {}",
                    counter_str.bright_black(),
                    optional_str,
                    name.green(),
                    vnum.yellow(),
                    vreq.bright_yellow(),
                    created.purple(),
                    updated.bright_purple(),
                    updated_ago_str.bright_purple(),
                    downloads.to_string().red(),
                    size_bytes,
                    size,
                    license,
                );

            }
        }

        Ok(())
    }

}
