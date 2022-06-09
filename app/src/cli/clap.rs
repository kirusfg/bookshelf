use clap::{arg, Command};

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

pub(crate) fn list_command() -> Command<'static> {
    Command::new("list")
        .about("List all of the entries on your bookshelf")
        .arg(arg!(-v - -verbose).required(false))
}

pub(crate) fn open_command() -> Command<'static> {
    Command::new("open")
        .about("Opens an entry in an external viewer")
        .arg(arg!(<ENTRY>).required(true).allow_invalid_utf8(true))
        .arg(
            arg!(-e --exec <PATH> "Sets a path to the executable to use")
                .required(false)
                .allow_invalid_utf8(true),
        )
}
