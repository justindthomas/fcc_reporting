#[macro_use]
extern crate rocket;

use std::path::Path;

use fcc_reporting::routes::focus::upload_focus_data;
use rocket::fs::{NamedFile, relative};

#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("static")).join("index.html");
    
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    rocket::build().mount("/", routes![index, upload_focus_data])
}
