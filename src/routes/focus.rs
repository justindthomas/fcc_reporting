use std::ops::Add;
use std::thread;
use std::io::prelude::*;
use std::fs::File;

use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::post;
use rocket::response::Redirect;
use serde::{Serialize, Deserialize};

use crate::analysis::{link, summarize_tracts, summarize_locations, TractSummationKey, Summation};
use crate::fcc::process_fcc_data;
use crate::emerald::{retrieve_subscriptions, ProductType, ServiceMedium};
use crate::reports::{broadband_subscription_report, voice_subscription_report, broadband_availability_report};

#[derive(FromForm)]
pub struct Upload<'r> {
    _save: bool,
    file: TempFile<'r>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct BroadbandStatistic {
    pub broadband_total: i32,
    pub broadband_consumer: i32,
    pub fttp: i32,
    pub fw: i32,
    pub copper: i32,
}

impl Add for BroadbandStatistic {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            broadband_total: self.broadband_total + other.broadband_total,
            broadband_consumer: self.broadband_consumer + other.broadband_consumer,
            fttp: self.fttp + other.fttp,
            fw: self.fw + other.fw,
            copper: self.copper + other.copper
        }
    }
}

pub type StatisticData = (TractSummationKey, Summation);

impl From<StatisticData> for BroadbandStatistic {
    fn from((key, summation): StatisticData) -> Self {
        Self {
            broadband_total: match key.product_type.clone() {
                ProductType::Internet(_) => summation.total.into(),
                _ => 0,
            },
            broadband_consumer: match key.product_type.clone() {
                ProductType::Internet(_) => summation.residential.into(),
                _ => 0,
            },
            fttp: match key.product_type.clone() {
                ProductType::Internet(profile) if profile.medium == ServiceMedium::Fiber => summation.total.into(),
                _ => 0,
            },
            fw: match key.product_type.clone() {
                ProductType::Internet(profile) if profile.medium == ServiceMedium::Wireless => summation.total.into(),
                _ => 0,
            },
            copper: match key.product_type {
                ProductType::Internet(profile) if profile.medium == ServiceMedium::Copper => summation.total.into(),
                _ => 0,
            }
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct VoipStatistic {
    pub voip_total: i32,
    pub voip_consumer: i32,
}

impl Add for VoipStatistic {
    type Output = Self;

    fn add (self, other: Self) -> Self {
        Self {
            voip_total: self.voip_total + other.voip_total,
            voip_consumer: self.voip_consumer + other.voip_consumer,
        }
    }
}

impl From<StatisticData> for VoipStatistic {
    fn from((key, summation): StatisticData) -> Self {
        Self {
            voip_total: match key.product_type.clone() {
                ProductType::Voip => summation.total.into(),
                _ => 0,
            },
            voip_consumer: match key.product_type.clone() {
                ProductType::Voip => summation.residential.into(),
                _ => 0,
            },
        }
    }
}

#[post("/focus", data = "<media>")]
pub async fn upload_focus_data(
    mut media: Form<Upload<'_>>,
) -> Redirect {

    let uuid = uuid::Uuid::new_v4().to_string().replace('-', "");

    let filename = format!("output/tmp/{uuid}");
    
    if media.file.persist_to(filename.clone()).await.is_ok() {
        thread::spawn(move || {
            let linked_records = link(process_fcc_data(filename.clone()), retrieve_subscriptions());
            let tract_summarization = summarize_tracts(linked_records.clone());
            let broadband_statistic = broadband_subscription_report(&uuid, &tract_summarization);
            let voip_statistic = voice_subscription_report(&uuid, &tract_summarization);
            
            let location_summarization = summarize_locations(linked_records.clone());
            broadband_availability_report(&uuid, &location_summarization);

            //log::debug!("ENTRIES: {}", linked_records.len());
            //log::debug!("TRACT SUMMARIZATION\n{tract_summarization:#?}");
            //log::debug!("LOCATION SUMMARIZATION\n{location_summarization:#?}");
            log::debug!("BROADBAND STATISTIC\n{broadband_statistic:#?}");
            log::debug!("VOIP STATISTIC\n{voip_statistic:#?}");
            
            std::fs::remove_file(filename).ok();

            let mut broadband_statistics_file = File::create(format!("output/reports/broadband_statistics-{uuid}.json")).unwrap();
            let _ = broadband_statistics_file.write_all(&serde_json::to_string(&broadband_statistic).unwrap().into_bytes());

            let mut voip_statistics_file = File::create(format!("output/reports/voip_statistics-{uuid}.json")).unwrap();
            let _ = voip_statistics_file.write_all(&serde_json::to_string(&voip_statistic).unwrap().into_bytes());
            
            log::debug!("THREAD COMPLETE");
        });
    }
    
    Redirect::to("/")
}
