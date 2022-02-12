#![feature(proc_macro_hygiene, decl_macro)]
// #![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use rocket::{get, post, request::Form, routes, FromForm};
use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct IndexConext {
    header: String,
}

#[derive(FromForm)]
struct User {
    username: String,
    password: String,
}

#[get("/")]
fn index() -> Template {
    let context = IndexConext {
        header: "Hello!".to_string(),
    };
    Template::render("index", &context)
}

#[post("/", data = "<userdata>")]
fn login(userdata: Form<User>) -> String {
    let res = format!("Hello, {}", userdata.username);
    let pass = userdata.password.clone();
    dbg!(pass);
    res
}

fn main() {
    initiate_logging();

    let my_routes = routes![index, login];
    dbg!(my_routes.clone());
    rocket::ignite()
        .mount("/", my_routes)
        .attach(Template::fairing())
        .launch();
}

fn initiate_logging() {
    // dotenv().ok();

    let env = dotenv::from_filename(".env").expect("'.env' not found.");
    dbg!(env);

    if std::env::var("PWD").is_err() {
        std::env::set_var("PWD", env!("CARGO_MANIFEST_DIR"));
        let pwd = std::env::var("PWD").unwrap();
        dbg!(pwd);
    }

    // std::env::set_var("RUST_LOG", "debug, actix_web=debug");
    // env_logger::init();
}
