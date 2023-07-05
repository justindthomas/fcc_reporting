use csv::ReaderBuilder;
use serde::Deserialize;

use std::fs::File;

#[derive(Debug, Deserialize, Clone)]
struct FccByteRecord<'a> {
    location_id: &'a [u8],
    address_primary: Option<&'a [u8]>,
    city: Option<&'a [u8]>,
    state: &'a [u8],
    zip: Option<u32>,
    zip_suffix: Option<u32>,
    unit_count: u64,
    bsl_flag: &'a [u8],
    building_type_code: char,
    land_use_code: u64,
    address_confidence_code: u64,
    county_geoid: &'a [u8],
    block_geoid: String,
    h3_9: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FccRecord {
    pub location_id: String,
    pub address_primary: Option<String>,
    pub city: Option<String>,
    pub state: String,
    pub zip: Option<u32>,
    pub zip_suffix: Option<u32>,
    pub unit_count: u64,
    pub bsl_flag: bool,
    pub building_type_code: char,
    pub land_use_code: u64,
    pub address_confidence_code: u64,
    pub county_geoid: String,
    pub block_geoid: String,
    pub h3_9: String,
    pub latitude: f64,
    pub longitude: f64,
}

impl<'a> From<FccByteRecord<'a>> for FccRecord {
    fn from(byte_record: FccByteRecord) -> Self {
        FccRecord {
            location_id: utf8(byte_record.clone().location_id),
            address_primary: byte_record.clone().address_primary.map(utf8),
            city: byte_record.clone().city.map(utf8),
            state: utf8(byte_record.state),
            zip: byte_record.zip,
            zip_suffix: byte_record.zip_suffix,
            unit_count: byte_record.unit_count,
            bsl_flag: utf8(byte_record.bsl_flag) == "TRUE",
            building_type_code: byte_record.building_type_code,
            land_use_code: byte_record.land_use_code,
            address_confidence_code: byte_record.address_confidence_code,
            county_geoid: utf8(byte_record.county_geoid),
            block_geoid: byte_record.block_geoid,
            h3_9: byte_record.h3_9,
            latitude: byte_record.latitude,
            longitude: byte_record.longitude,
        }
    }
}

fn utf8(bytes: &[u8]) -> String {
    encoding_rs::mem::decode_latin1(bytes).into_owned()
}

pub fn process_fcc_data(filename: String) -> Vec<FccRecord> {
    let file = File::open(filename).unwrap();
    let mut rdr = ReaderBuilder::new().from_reader(&file);

    rdr.byte_records()
        .filter_map(|byte_record| {
            if let Ok(byte_record) = byte_record {
                match byte_record.deserialize::<FccByteRecord>(None) {
                    Ok(fcc_record) => Some(fcc_record.into()),
                    Err(e) => {
                        log::error!("{e}");
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect()
}
