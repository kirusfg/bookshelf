/// Clap-related commands and arguments for the CLI
pub(crate) mod clap;

use ::clap::ArgMatches;

use lib::entry::Entry;

use crate::{app::App, utils::format::format_entry};

// Extracts a file path from the matches provided, constructs an entry from
// it and adds it to the bookshelf.
pub(crate) fn add_entry(app: &mut App, matches: &ArgMatches) {
    let mut entry =
        Entry::new(matches.value_of_os("FILE").unwrap().to_str().unwrap());

    if let Some(bib_path) = matches.value_of_os("bib") {
        entry = entry.with_bib(bib_path.to_str().unwrap());
    }

    // New entry index
    let entry_index = app.size() + 1;

    let entry_name = format_entry(entry_index, &entry);

    match app.add_entry(&entry) {
        Ok(()) => {
            println!("Successfully added '{}'", entry_name)
        }
        Err(e) => {
            println!("Couldn't add '{}': {}", entry_name, e)
        }
    }
}

/// Extracts an entry index from the matches provided, and removes the
/// corresponding entry from the bookshelf, if it exists.
pub(crate) fn remove_entry(app: &mut App, matches: &ArgMatches) {
    let entry_index =
        matches.value_of("INDEX").unwrap().parse::<usize>().unwrap();

    let entry = app.shelf.get_index(entry_index);

    match entry {
        Ok(entry) => {
            let entry_name = format_entry(entry_index, entry);

            match app.remove_entry_index(entry_index) {
                Ok(()) => println!("Successfully removed '{}''", entry_name),
                Err(e) => println!("Couldn't remove '{}': {}", entry_name, e),
            }
        }
        Err(e) => println!("Couldn't remove entry {}: {}", entry_index, e),
    }
}

/// Extracts an entry index from the matches provided, and opens the
/// corresponding entry via platform-specific default program, or a program
/// specified by the user, if the entry exists.
pub(crate) fn open_entry(app: &mut App, matches: &ArgMatches) {
    let entry_index =
        matches.value_of("INDEX").unwrap().parse::<usize>().unwrap();

    let entry = app.shelf.get_index(entry_index);

    match entry {
        Ok(entry) => {
            let entry_name = format_entry(entry_index, entry);

            let exe = matches
                .value_of_os("exec")
                .map(|os_str| String::from(os_str.to_str().unwrap()));

            match app.open_entry(entry, exe.clone()) {
                Ok(()) if exe.is_some() => println!(
                    "Successfully opened '{}' in {}",
                    entry_name,
                    exe.unwrap()
                ),
                Ok(()) => println!("Successfully opened '{}'", entry_name),
                Err(_) if exe.is_some() => {
                    println!(
                        "Couldn't open '{}': {} not found",
                        entry_name,
                        exe.unwrap()
                    )
                }
                Err(e) => {
                    println!("Couldn't open '{}': {}", entry_name, e)
                }
            }
        }
        Err(e) => println!("Couldn't open entry {}: {}", entry_index, e),
    }
}

/// Simply lists all of the entries on the bookshelf. The output format
/// depends on whether the entry has a bibliography file associated with
/// it or not.
pub(crate) fn list_entries(app: &mut App, _matches: &ArgMatches) {
    // TODO: parse matches for output options
    for (i, entry) in app.list_entries() {
        let entry_name = format_entry(i + 1, entry);
        println!("{}", entry_name);
    }
}
