use clap::{
    arg, crate_authors, crate_description, crate_name, crate_version, Command,
};

fn main() {
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            arg!(-c --config <FILE> "Sets a path to a config file")
                .required(false)
                .allow_invalid_utf8(true),
        )
        .subcommand(
            Command::new("add")
                .about("Adds an entry to your bookshelf")
                .arg(arg!(<FILE>))
                .arg(
                    arg!(-b --bib <FILE> "Sets a path to the bibliographic entry")
                        .required(false)
                        .allow_invalid_utf8(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        if matches.is_present("FILE") {
            println!("Entry was added");
        }
    }
}
