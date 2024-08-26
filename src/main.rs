static mut VALIDATED: bool = false;

#[macro_use] extern crate rocket;

#[get("/")]
fn auth() -> rocket::response::content::RawHtml<String> {
    rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
}

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

#[get("/process_new_product/<product_name>/<cost>")]
fn process_new_product(product_name: &str, cost: f32) { //-> rocket::response::content::RawHtml<String> {
    println!("\nProduct name: {}", product_name);
    println!("Cost: {}\n", cost);
//    rocket::response::content::RawHtml(std::fs::read_to_string("xhtml/auth.xhtml").unwrap())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![auth, index, process, products, new_product, process_new_product])
}

