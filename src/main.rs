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
use rocket::{response::Redirect, request::Form};
use rocket_contrib::templates::Template;
use schema::users::password;
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


#[post("/newuser", data = "<user>")]
fn create_user(conn: DbConn, user:Form<models::NewUser>) -> Template {
    let u=user.0.clone();
    println!("before{:#?}",user.0);
    
    let _ = diesel::insert_into(users::table)
        .values(user.0)
        .execute(&*conn);
        println!("after{:#?}",u);
   // Redirect::to("/users")
       let mut context: std::collections::HashMap<String, String> = std::collections::HashMap::new();
context.insert("username".to_string(), u.username);
    Template::render("user", context)
}



#[get("/newuser")]
fn new_user_page() -> Template {
    let context: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    Template::render("new_user", context)
}
#[derive(Serialize)]
struct IndexConext {
    header: String,
}



#[get("/")]
fn index() -> Template {
    let context = IndexConext {
        header: "Hello!".to_string(),
    };
    Template::render("index", &context)
}

#[post("/", data = "<user>")]
fn login(conn:DbConn,user: Form<models::NewUser>) -> String {
    let res = format!("Hello, {}", user.username);
    let pass = user.password.clone();
    let u=user.0.clone();
    println!("before{:#?}",user.0);
    
    let _ = diesel::insert_into(users::table)
        .values(user.0)
        .execute(&*conn);
        println!("after{:#?}",u);
 
    dbg!(pass);
    res
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .mount(
            "/",
            routes![
                index,
                login,
                get_user_info,
                get_users,
                new_user_page
            ],
        ).mount(
            "/newuser",
            routes![
                 create_user,
                
            ],
        )
        .launch();
}
