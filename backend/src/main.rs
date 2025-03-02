#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Artfolio API"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
