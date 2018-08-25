#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::{NamedFile, Redirect, Flash};
use rocket::http::{RawStr, Status, Cookies, Cookie};
use rocket::request::{self, Request, FromRequest, Form};
use std::path::{Path, PathBuf};
use std::io;
use rocket::Outcome;


#[derive(FromForm)]
struct LoginFormInput<'ab> {
    login: &'ab RawStr,
    password: &'ab RawStr
}

struct UserAdmin(String);

impl<'a, 'r> FromRequest<'a, 'r> for UserAdmin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserAdmin, ()> {
        // let keys: Vec<_> = request.headers().get("x-api-key").collect();
        // if keys.len() != 1 {
        //     return Outcome::Failure((Status::BadRequest, ()));
        // } 


        // let key = keys[0];
        // if !is_valid(keys[0]) {
        //     return Outcome::Forward(());
        // }

        return Outcome::Success(UserAdmin("AdminKey".to_string()));
    }
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("client/pages/index.html")
}

#[get("/check")]
fn check(mut cookies: Cookies) -> Option<String> {
    cookies.get_private("user_id")
        .map(|cookie| format!("User ID: {}", cookie.value()))
}

#[get("/cookies")]
fn get_cookies(mut cookies: Cookies) -> String {
    cookies.add_private(Cookie::new("user_id", "kekkonen"));
    format!("Created a cookie for you!")
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/login"), "Successfully logged out.")
}

// #[get("/customer")]
// fn customer(session: Session) -> io::Result<NamedFile> {
//     NamedFile::open("client/pages/customer.html")
//     // Return normal page 
// }

#[get("/login")]
fn login() -> io::Result<NamedFile> {
    NamedFile::open("client/pages/login.html")
}

#[post("/login", data = "<login_input>")]
fn login_submit<'r>(login_input: Form<'r, LoginFormInput<'r>>) -> io::Result<NamedFile> {
    println!("Hello, {}, with password {}!", login_input.get().login, login_input.get().password);
    NamedFile::open("client/pages/login.html")
    // Return session
}

#[get("/src/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/src/").join(file)).ok()
}

/////////////////////////////////////////// API
#[get("/api/secret")]
fn secret_info(admin: UserAdmin) -> String {
    format!("Lol!, Secret info! key: {}", admin.0)
}

fn main() {
    rocket::ignite().mount("/", routes![index, files, login, login_submit, secret_info, check, logout, get_cookies]).launch();
}
