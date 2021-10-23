use clap::{crate_authors, crate_version, App, Arg, SubCommand};

pub use lib;

fn main() {
    let matches = App::new("bookshelf")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Command-line interface for managing your local library")
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a book or an article file to the repository")
                .arg(
                    Arg::with_name("file")
                        .help("Book or article file")
                        .index(1)
                        .takes_value(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => lib::add_file(sub_m.value_of("file").unwrap().to_string()),
        _ => println!("Unknown command"),
    };
}
