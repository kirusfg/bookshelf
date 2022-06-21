use lib::entry::Entry;

pub(crate) fn format_entry(entry_index: usize, entry: &Entry) -> String {
    let bib_entry = entry.get_bib_entry();

    let entry_file_name = if bib_entry.is_some() {
        // TODO: make use of the BibTeX file to name the entry
        todo!()
    } else {
        entry
            .path
            .file_name()
            .expect("The file has been validated and must have a name")
            .to_str()
            .unwrap()
    };

    format!("{} - {}", entry_index, entry_file_name)
}
