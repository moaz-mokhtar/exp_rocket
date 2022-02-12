#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod hbs;

// use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

// #[get("/")]
// fn index() -> RawHtml<&'static str> {
//     RawHtml(r#"See <a href="hbs">Handlebars</a>."#)
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        // .mount("/", routes![index])
        .mount("/", routes![hbs::index, hbs::hello, hbs::about])
        .register("/", catchers![hbs::not_found])
        .attach(Template::custom(|engines| {
            hbs::customize(&mut engines.handlebars);
        }))
}

// fn main() -> _ {

// }
