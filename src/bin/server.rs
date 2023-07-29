#[macro_use] extern crate rocket;

use std::path::Path;

use fcc_reporting::routes::{focus::upload_focus_data, reports::{report_list, statistics_list}};
use rocket::fs::{NamedFile, FileServer};

#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new("static").join("index.html");
    
    NamedFile::open(path).await.ok()
}

#[launch]
fn rocket() -> _ {
    env_logger::init();

    rocket::build()
        .mount("/static", FileServer::from("static"))
        .mount("/report", FileServer::from("output/reports"))
        .mount("/", routes![
            index,
            upload_focus_data,
            report_list,
            statistics_list
        ])
}
