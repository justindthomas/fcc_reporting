use std::thread;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::post;
use rocket::response::Redirect;

use crate::analysis::{link, summarize_tracts, summarize_locations};
use crate::fcc::process_fcc_data;
use crate::emerald::retrieve_subscriptions;
use crate::reports::{broadband_subscription_report, voice_subscription_report, broadband_availability_report};

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
            let tract_summarization = summarize_tracts(linked_records.clone());
            broadband_subscription_report(&tract_summarization);
            voice_subscription_report(&tract_summarization);

            let location_summarization = summarize_locations(linked_records.clone());
            broadband_availability_report(&location_summarization);

            log::debug!("ENTRIES: {}", linked_records.len());
            log::debug!("TRACT SUMMARIZATION\n{tract_summarization:#?}");
            log::debug!("LOCATION SUMMARIZATION\n{location_summarization:#?}");
            
            std::fs::remove_file(filename).ok();
            log::debug!("THREAD COMPLETE");
        });
    }
    
    Redirect::to("/")
}
