pub fn add_file(file: String) {
    println!("Added file {}", file)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
