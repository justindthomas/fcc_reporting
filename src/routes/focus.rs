use std::thread;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::post;
use rocket::response::Redirect;

use crate::analysis::{link, summarize};
use crate::fcc::process_fcc_data;
use crate::emerald::retrieve_subscriptions;
use crate::reports::broadband_subscription_report;

#[derive(FromForm)]
pub struct Upload<'r> {
    _save: bool,
    file: TempFile<'r>,
}

#[post("/focus", data = "<media>")]
pub async fn upload_focus_data(
    mut media: Form<Upload<'_>>,
) -> Redirect {

    let uuid = uuid::Uuid::new_v4().to_string();

    let filename = format!("output/tmp/{uuid}");

    if media.file.persist_to(filename.clone()).await.is_ok() {
        thread::spawn(|| {
            let linked_records = link(process_fcc_data(filename.clone()), retrieve_subscriptions());
            let summarization = summarize(linked_records.clone());
            broadband_subscription_report(&summarization);

            log::debug!("ENTRIES: {}", linked_records.len());
            log::debug!("SUMMARIZATION\n{summarization:#?}");
            
            std::fs::remove_file(filename).ok();
            log::debug!("THREAD COMPLETE");
        });
    }
    
    Redirect::to("/")
}
