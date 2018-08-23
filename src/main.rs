#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::response::NamedFile;
use std::path::{Path, PathBuf};
use std::io;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("client/dist/index.html")
    // format!("Hello, {} year old named {}!", age, name)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("client/dist/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, files]).launch();
}