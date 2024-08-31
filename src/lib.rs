pub mod files {

    // returns true if "item" exists in "filename" in any line at "position"
    pub fn item_exists(filename: String, position: usize, item: String) -> bool {
        let filename_lines: Vec<String> = std::fs::read_to_string(filename)
            .unwrap()
            .split("\n")
            .map(|it| it.to_string())
            .filter(|it| !it.is_empty())
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

    pub fn clean_file(file_name: String) {
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file_name)
            .unwrap();
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

    pub fn sku_list() {
        clean_file("xhtml/tmp_sku_list.html".to_string());
        let first_half: String = std::fs::read_to_string("xhtml/sku_list_first_half.html").unwrap();
        append_to_file("xhtml/tmp_sku_list.html".to_string(), first_half);
        let product_lines: Vec<String> = std::fs::read_to_string("txt/products.txt")
            .unwrap()
            .split("\n")
            .filter(|it| !it.is_empty())
            .map(|it| it.to_string())
            .collect();
        for line in product_lines {
            let parts: Vec<String> = line
                .split(";")
                .map(|it| it.to_string())
                .collect();
            let product = parts[0].clone();
            let sku = parts[1].clone();
            let mut sku_html_line = format!("<li><input type=\"checkbox\" id=\"{sku}\" value=\"{sku}\">");
            append_to_file("xhtml/tmp_sku_list.html".to_string(), sku_html_line);
            sku_html_line = format!("<label for=\"{sku}\">SKU: {sku}</label><h3>Product: {product}</h3></li>");
            append_to_file("xhtml/tmp_sku_list.html".to_string(), sku_html_line);
        }
        let second_half: String = std::fs::read_to_string("xhtml/sku_list_second_half.html").unwrap();
        append_to_file("xhtml/tmp_sku_list.html".to_string(), second_half);

        std::fs::rename("xhtml/tmp_sku_list.html", "xhtml/sku_list.xhtml").unwrap();
    }
}
