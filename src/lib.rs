pub mod files {

    // returns true if item exists in filename
    pub fn item_exists(filename: String, item: String) -> bool {
        todo!()
    }

    pub fn append_to_file(filename: String, data: String) {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();
        write!(file, "{}\n", &data).expect("Unable to write data");
    }

}
