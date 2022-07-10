use clap::{arg, crate_description, crate_name, crate_version, Command};

pub(crate) fn get_cli_commands() -> Command<'static> {
    Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .subcommand(add_command())
        .subcommand(remove_command())
        .subcommand(open_command())
        .subcommand(list_command())
}

pub(crate) fn add_command() -> Command<'static> {
    Command::new("add")
        .about("Adds an entry to your bookshelf")
        .arg(arg!(<FILE>).allow_invalid_utf8(true))
        .arg(
            arg!(-b --bib <FILE> "Sets a path to the Bib(La)TeX entry")
                .required(false)
                .allow_invalid_utf8(true),
        )
}

pub(crate) fn remove_command() -> Command<'static> {
    Command::new("remove")
        .about("Removes an entry from your bookshelf (by its index)")
        .arg(arg!(<INDEX>).required(true))
}

pub(crate) fn list_command() -> Command<'static> {
    Command::new("list")
        .about("List all of the entries on your bookshelf")
        .arg(arg!(-v - -verbose).required(false))
}

pub(crate) fn open_command() -> Command<'static> {
    Command::new("open")
        .about("Opens an entry in an external viewer")
        .arg(
            arg!(<INDEX> "Numerical index of the entry in the bookshelf")
                .required(true),
        )
        .arg(
            arg!(-e --exec <EXE> "Sets the executable to use")
                .required(false)
                .allow_invalid_utf8(true),
        )
}
