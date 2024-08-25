#[macro_use] extern crate rocket;

#[get("/")]
fn auth() -> rocket::response::content::RawHtml<String> {
    rocket::response::content::RawHtml(std::fs::read_to_string("auth.xhtml").unwrap())
}

#[get("/index")]
fn index() -> rocket::response::content::RawHtml<String> {
    rocket::response::content::RawHtml(std::fs::read_to_string("hello.xhtml").unwrap())
}

#[get("/process/<password>")]
fn process(password: &str) -> rocket::response::Redirect {
    println!("\nProcesando password: {}\n", password);
    if password == "26082011" {
        rocket::response::Redirect::to(uri!(index))
    } else {
        rocket::response::Redirect::to(uri!(auth))
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![auth, index, process])
}

