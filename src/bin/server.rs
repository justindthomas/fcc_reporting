#[macro_use]
extern crate rocket;

use rocket::Data;
use rocket::{data::ToByteUnit, post};
use rocket::http::Status;
use rocket::form::Form;
use rocket::fs::TempFile;

use csv::ReaderBuilder;

use std::fs::File;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(FromForm)]
pub struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[post("/focus", data = "<media>")]
pub async fn upload_focus_data(
    mut media: Form<Upload<'_>>,
) -> Result<Status, Status> {

    media.file.persist_to("./fcc.csv").await.ok();

    let file = File::open("./fcc.csv").unwrap();

    let mut rdr = ReaderBuilder::new()
        .from_reader(&file);

    for result in rdr.records().flatten() {
        log::debug!("{result:#?}");
    };

    Ok(Status::Accepted)
    
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    
    rocket::build().mount("/", routes![index, upload_focus_data])
}
