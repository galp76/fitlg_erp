static mut VALIDATED: bool = false;

#[macro_use] extern crate rocket;

// Pagina de login
#[get("/")]
fn auth() -> rocket::response::content::RawHtml<String> {
    rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
}

// Menu principal
#[get("/index")]
fn index() -> rocket::response::content::RawHtml<String> {
    unsafe {
        if VALIDATED == true {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/index.xhtml").unwrap())
        } else {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        }
    }
}

// Procesa la contrase#a
#[get("/process/<password>")]
fn process(password: &str) -> rocket::response::Redirect {
    println!("\nProcesando password: {}\n", password);
    if password == "26082011" {
        unsafe {
            VALIDATED = true;
        }
        rocket::response::Redirect::to(uri!(index))
    } else {
        rocket::response::Redirect::to(uri!(auth))
    }
}

// Pagina principal de productos
#[get("/products")]
fn products() -> rocket::response::content::RawHtml<String> {
    unsafe {
        if VALIDATED == true {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/products.xhtml").unwrap())
        } else {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        }
    }
}

// Formulario para nuevo producto
#[get("/new_product")]
fn new_product() -> rocket::response::content::RawHtml<String> {
    unsafe {
        if VALIDATED == true {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/new_product.xhtml").unwrap())
        } else {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        }
    }
}

// Procesa el formulario paara nuevo producto
#[get("/process_new_product/<product_name>/<sku>/<categories>/<description>/<unit_of_measure>/<cost>/<retail>/<discounts>")]
fn process_new_product(product_name: &str, sku: &str, categories: &str, description: &str, unit_of_measure: &str, cost: &str, retail: &str, discounts: &str) -> rocket::response::Redirect {
    unsafe {
        if VALIDATED == true {
            if !fitlg_erp::files::item_exists("txt/products.txt".to_string(), 1, sku.to_string()) {
                let data = format!("{};{};{};{};{};{};{};{}",
                    product_name,
                    sku,
                    categories,
                    description,
                    unit_of_measure,
                    cost,
                    retail,
                    discounts
                );
                fitlg_erp::files::append_to_file("txt/products.txt".to_string(), data);
                rocket::response::Redirect::to(uri!(product_added))
            } else {
                rocket::response::Redirect::to(uri!(sku_already_exists))
            } 
        } else {
            rocket::response::Redirect::to(uri!(auth))
        }
    }
}

// Notifica sobre nuevo producto agregado
#[get("/product_added")]
fn product_added() -> rocket::response::content::RawHtml<String> {
    unsafe {
        if VALIDATED == true {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/product_added.xhtml").unwrap())
        } else {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        }
    }
}

// Informa que el SKU usado en el formulario ya existe
#[get("/sku_already_exists")]
fn sku_already_exists() -> rocket::response::content::RawHtml<String> {
    unsafe {
        if VALIDATED == true {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/sku_already_exists.xhtml").unwrap())
        } else {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        }
    }
}

// Presenta una lista de SKU's para seleccionar uno
#[get("/read_product")]
fn read_product() -> rocket::response::content::RawHtml<String> {
    unsafe {
        if VALIDATED == true {
            fitlg_erp::files::sku_list();
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/sku_list.xhtml").unwrap())
        } else {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        }
    }
}

// Muestra la informacion correspondiente al SKU indicado
#[get("/sku_read/<sku>")]
fn sku_read(sku: String) -> rocket::response::content::RawHtml<String> {
    // Buscamoss la linea que contiene el SKU suministrado
    let products_line: Vec<String> = std::fs::read_to_string("txt/products.txt")
        .unwrap()
        .split("\n")
        .map(|it| it.to_string())
        .filter(|it| !it.is_empty())
        .collect();

    let mut sku_line = "".to_string();
    for line in products_line {
        let parts: Vec<String> = line
            .split(";")
            .map(|it| it.to_string())
            .collect();
        if parts[1] == sku {
            // Construimos el archivo sku_information.xhtml 
            fitlg_erp::files::clean_file("xhtml/tmp_sku_information.xhtml".to_string());
            let first_half: String = std::fs::read_to_string("xhtml/sku_information_first_half.xhtml").unwrap();
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), first_half);

            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>Nombre del producto: {}</h3></li>", parts[0]));
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>SKU: {}</h3></li>", parts[1]));
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>Categorias: {}</h3></li>", parts[2]));
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>Descripcion: {}</h3></li>", parts[3]));
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>Unidad de medida: {}</h3></li>", parts[4]));
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>Costo: {}</h3></li>", parts[5]));
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>Precio de venta: {}</h3></li>", parts[6]));
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), format!("<li><h3>Descuentos: {}</h3></li>", parts[7]));

            let second_half: String = std::fs::read_to_string("xhtml/sku_information_second_half.xhtml").unwrap();
            fitlg_erp::files::append_to_file("xhtml/tmp_sku_information.xhtml".to_string(), second_half);

            std::fs::rename("xhtml/tmp_sku_information.xhtml", "xhtml/sku_information.xhtml").unwrap();
            break;
        }
    }

    rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/sku_information.xhtml").unwrap())
}

// Log out
#[get("/log_out")]
fn log_out() -> rocket::response::content::RawHtml<String> {
    unsafe {
        if VALIDATED == true {
            VALIDATED = false;
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        } else {
            rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![auth, index, process, products, new_product, process_new_product, product_added, sku_already_exists, read_product, sku_read, log_out])
}

