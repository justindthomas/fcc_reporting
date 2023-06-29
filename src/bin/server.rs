#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::Data;
use rocket::{data::ToByteUnit, post};

use csv::ReaderBuilder;

use std::fs::File;

use fcc_reporting::routes::focus::upload_focus_data;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    rocket::build().mount("/", routes![index, upload_focus_data])
}
