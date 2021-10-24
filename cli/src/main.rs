use clap::{crate_authors, crate_version, App, Arg, SubCommand};

mod entry;
use entry::add_entry;

fn main() {
    // TODO: Move this to another module and generate completion files for different shells at compile time
    let matches = App::new("bookshelf")
        .version(crate_version!())
        .author(crate_authors!())
        .bin_name("bookshelf")
        .about("Command-line interface for managing your local library")
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds a new item to your bookshelf")
                .arg(
                    Arg::with_name("kind")
                        .help("Kind of item to be added")
                        .index(1)
                        .takes_value(true)
                        .required(true)
                        .possible_values(&["book", "article"]),
                )
                .arg(
                    Arg::with_name("file")
                        .help("Path to the file")
                        .index(2)
                        .takes_value(true)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("add", Some(sub_m)) => add_entry(
            &sub_m.value_of("kind").unwrap().to_string(),
            &sub_m.value_of("file").unwrap().to_string(),
        ),
        _ => println!("Unknown command"),
    };
}
