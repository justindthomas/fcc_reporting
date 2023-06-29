use std::thread;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::post;

use crate::analysis::link;
use crate::fcc::process_fcc_data;
use crate::emerald::retrieve_subscriptions;

#[derive(FromForm)]
pub struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[post("/focus", data = "<media>")]
pub async fn upload_focus_data(
    mut media: Form<Upload<'_>>,
) -> Result<Status, Status> {

    let uuid = uuid::Uuid::new_v4().to_string();

    let filename = format!("output/tmp/{uuid}");

    if media.file.persist_to(filename.clone()).await.is_ok() {
        thread::spawn(|| {
            let linked_records = link(process_fcc_data(filename.clone()), retrieve_subscriptions());

            log::debug!("ENTRIES: {}", linked_records.len());
            
            std::fs::remove_file(filename).ok();
            log::debug!("THREAD COMPLETE");
        });
    }
    
    Ok(Status::Accepted)
}
