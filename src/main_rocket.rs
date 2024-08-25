use rocket::{get, routes};

#[get("/")]
fn index() -> rocket::response::content::RawHtml<String> {
    rocket::response::content::RawHtml(std::fs::read_to_string("hello.xhtml").unwrap())
}

#[get("/re/<minutes>")]
fn remos(minutes: &str) -> /*rocket::response::content::RawHtml<String>*/ rocket::response::Redirect {
    gllrrocket::data::newData("r", minutes); 
    rocket::response::Redirect::to(rocket::uri!(index))
//    rocket::response::content::RawHtml(std::fs::read_to_string("hello.xhtml").unwrap())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index, remos]);

    Ok(rocket.into())
}
