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

            /* OOOOOOOJJJJJJOOOOOOO 
             * AQUI LO PRIMERO QUE HAY QUE HACER ES CCHEQUEAR QUE EL 
             * SKU NO EXISTA, HAY QUE ESCRIBIR LA FUNCION */

            println!("\nProduct name: {}", product_name);
            println!("Cost: {}\n", cost);
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
            rocket::response::Redirect::to(uri!(index))
        } else {
            rocket::response::Redirect::to(uri!(auth))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![auth, index, process, products, new_product, process_new_product])
}

