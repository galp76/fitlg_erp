fn main() {
    sku_list();
}

fn sku_list() {
    sku_tests::files::clean_file("xhtml/tmp_sku_list.html".to_string());
    let first_half: String = std::fs::read_to_string("xhtml/sku_list_first_half.html").unwrap();
    sku_tests::files::append_to_file("xhtml/tmp_sku_list.html".to_string(), first_half);
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
        println!("{:?}", parts);
        let product = parts[0].clone();
        let sku = parts[1].clone();
        let mut sku_html_line = format!("<li><input type=\"checkbox\" id=\"{sku}\" value=\"{sku}\">");
        sku_tests::files::append_to_file("xhtml/tmp_sku_list.html".to_string(), sku_html_line);
        sku_html_line = format!("<label for=\"{sku}\">SKU: {sku}</label><h3>Product: {product}</h3></li>");
        sku_tests::files::append_to_file("xhtml/tmp_sku_list.html".to_string(), sku_html_line);
    }
    let second_half: String = std::fs::read_to_string("xhtml/sku_list_second_half.html").unwrap();
    sku_tests::files::append_to_file("xhtml/tmp_sku_list.html".to_string(), second_half);

    std::fs::rename("xhtml/tmp_sku_list.html", "xhtml/sku_list.html").unwrap();
}
