pub mod files {

    // returns true if "item" exists in "filename" in any line at "position"
    pub fn item_exists(filename: String, position: usize, item: String) -> bool {
        let filename_lines: Vec<String> = std::fs::read_to_string(filename)
            .unwrap()
            .split("\n")
            .map(|it| it.to_string())
            .collect();

        for line in filename_lines {
            let line_items: Vec<String> = line
                .split(";")
                .map(|it| it.to_string())
                .collect();

            if line_items[position] == item {
                return true;
            }
        }

        false
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
