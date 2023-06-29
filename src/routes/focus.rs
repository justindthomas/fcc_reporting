use rocket::form::Form;
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::post;

use csv::ReaderBuilder;
use serde::Deserialize;

use std::fs::{File, self};

#[derive(FromForm)]
pub struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[derive(Debug, Deserialize, Clone)]
struct FccByteRecord<'a> {
    location_id: String,
    address_primary: Option<&'a [u8]>,
    city: String,
    state: String,
    zip: String,
    zip_suffix: Option<String>,
    unit_count: u64,
    bsl_flag: String,
    building_type_code: String,
    land_use_code: u64,
    address_confidence_code: u64,
    county_geoid: u64,
    block_geoid: String,
    h3_9: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize, Clone)]
struct FccRecord {
    location_id: String,
    address_primary: Option<String>,
    city: String,
    state: String,
    zip: String,
    zip_suffix: Option<String>,
    unit_count: u64,
    bsl_flag: String,
    building_type_code: String,
    land_use_code: u64,
    address_confidence_code: u64,
    county_geoid: u64,
    block_geoid: String,
    h3_9: String,
    latitude: f64,
    longitude: f64,
}

impl<'a> From<FccByteRecord<'a>> for FccRecord {
    fn from(byte_record: FccByteRecord) -> Self {
        FccRecord {
            location_id: byte_record.clone().location_id,
            address_primary: byte_record.clone().address_primary.map(utf8),
            city: byte_record.city,
            state: byte_record.state,
            zip: byte_record.zip,
            zip_suffix: byte_record.zip_suffix,
            unit_count: byte_record.unit_count,
            bsl_flag: byte_record.bsl_flag,
            building_type_code: byte_record.building_type_code,
            land_use_code: byte_record.land_use_code,
            address_confidence_code: byte_record.address_confidence_code,
            county_geoid: byte_record.county_geoid,
            block_geoid: byte_record.block_geoid,
            h3_9: byte_record.h3_9,
            latitude: byte_record.latitude,
            longitude: byte_record.longitude
        }
    }
}

fn utf8(bytes: &[u8]) -> String {
    encoding_rs::mem::decode_latin1(bytes).into_owned()
}

fn process_fcc_data(filename: String) -> Vec<FccRecord> {
    let file = File::open(filename).unwrap();
    let mut rdr = ReaderBuilder::new().from_reader(&file);

    rdr.byte_records().filter_map(|byte_record| {
        if let Ok(byte_record) = byte_record {
            if let Ok(fcc_record) = byte_record.deserialize::<FccByteRecord>(None) {
                Some(fcc_record.into())
            } else {
                None
            }
        } else {
            None
        }
    }).collect()
}

#[post("/focus", data = "<media>")]
pub async fn upload_focus_data(
    mut media: Form<Upload<'_>>,
) -> Result<Status, Status> {

    let uuid = uuid::Uuid::new_v4().to_string();

    let filename = format!("output/tmp/{uuid}");

    if media.file.persist_to(filename.clone()).await.is_ok() {
        process_fcc_data(filename.clone());
        fs::remove_file(filename).ok();
    }
    
    Ok(Status::Accepted)   
}
