#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use std::vec;

use country_emoji::flag;
use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};

pub mod models;
pub mod schema;

use self::schema::*;

#[database("database")]
struct DbConn(rocket_contrib::databases::diesel::MysqlConnection);

#[derive(Serialize, Deserialize)]
struct TeraUser {
    user: models::User,
}

#[get("/user/<username>")]
fn get_user_info(conn: DbConn, username: String) -> Template {
    let user: Result<models::User, _> = users::table
        .filter(users::username.eq(&username))
        .get_result(&*conn);

    match user {
        Ok(user) => {
        
            let mut context = TeraUser {
                user: user,
            };
           

            Template::render("user", context)
        }
        _ => {
            let context: std::collections::HashMap<String, String> =
                std::collections::HashMap::new();
            Template::render("user_not_found", context)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TeraUsers {
    users: Vec<models::User>,
}

#[get("/users")]
fn get_users(conn: DbConn) -> Template {
    let results: Result<Vec<models::User>, _> = users::table.get_results(&*conn);
      

    let mut context = TeraUsers { users: vec![] };
    for mut u in results.unwrap() {
        context.users.push(u);
    }

    Template::render("users", context)
}


#[post("/newuser", data = "<NewUser_form>")]
fn create_user(conn: DbConn, user:Form<NewUser>) -> Redirect {
    
    let _ = diesel::insert_into(users::table)
        .values(user)
        .execute(&*conn);

    Redirect::to("/users")
}

#[get("/newuser")]
fn new_user_page() -> Template {
    let context: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    Template::render("new_user", context)
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                get_user_info,
                get_users,
                create_user,
                new_user_page
            ],
        )
        .launch();
}
