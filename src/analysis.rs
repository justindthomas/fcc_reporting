use std::collections::HashMap;

use crate::{
    emerald::{ProductType, SubscriptionApiItem, PRODUCT_CODES},
    fcc::FccRecord,
};
use fuzzywuzzy::fuzz;

fn digits(text: String) -> String {
    text.chars().filter(|c| c.is_ascii_digit()).collect()
}

pub fn link(
    fcc: Vec<FccRecord>,
    emerald: Vec<SubscriptionApiItem>,
) -> Vec<(FccRecord, SubscriptionApiItem)> {
    emerald
        .iter()
        .filter_map(|x| {
            let mut linked: Option<(FccRecord, SubscriptionApiItem)> = None;

            for y in fcc.clone() {
                if let (
                    Some(emerald_address),
                    Some(emerald_city_state_zip),
                    Some(fcc_address),
                    Some(fcc_city),
                ) = (
                    x.customer.cf_service_address.clone(),
                    x.customer.cf_service_city_st_zip.clone(),
                    y.address_primary.clone(),
                    y.city.clone(),
                ) {
                    if let (
                        Some((emerald_numbers, emerald_street)),
                        Some((emerald_city, _emerald_other)),
                        Some((fcc_numbers, fcc_street)),
                    ) = (
                        emerald_address.clone().split_once(' '),
                        emerald_city_state_zip.split_once(','),
                        fcc_address.clone().split_once(' '),
                    ) {
                        if emerald_city.to_uppercase() == fcc_city.to_uppercase()
                            && digits(emerald_address.clone()) == digits(fcc_address.clone())
                            && emerald_numbers == fcc_numbers
                            && fuzz::ratio(
                                &emerald_street.to_uppercase(),
                                &fcc_street.to_uppercase(),
                            ) > 80
                        {
                            linked = Some((y, x.clone()));
                            log::debug!("MATCH: {} => {}", emerald_address, fcc_address);
                            break;
                        }
                    }
                }
            }

            linked
        })
        .collect()
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct SummationKey {
    pub tract_id: String,
    pub product_type: ProductType,
}

#[derive(Debug)]
pub struct Summation {
    pub total: u16,
    pub residential: u16,
}

fn is_consumer(text: String) -> bool {
    text.to_uppercase() == "RESIDENTIAL"
        || text.to_uppercase() == "RESIDENTAIL"
        || text.to_uppercase() == "RRESIDENTIAL"
}

// I'm using and mutating the weird "111,222,333,444,555" string from
// the FCC report to classify the tracts. I don't know why they don't
// just include an eleven character version since that's what they want
// to see in the submissions
fn get_tract(text: String) -> String {
    let mut pruned = text.replace(['\"', ','], "");
    pruned.truncate(11);

    pruned
}

pub fn summarize(
    linked_records: Vec<(FccRecord, SubscriptionApiItem)>,
) -> HashMap<SummationKey, Summation> {
    let mut summarization: HashMap<SummationKey, Summation> = HashMap::new();

    for (fcc, emerald) in linked_records {
        if let Some(plan_id) = emerald.subscription.plan_id {
            if let (Some(product_type), Some(cf_residentialbusiness)) = (
                (*PRODUCT_CODES).get(&plan_id),
                emerald.customer.cf_residentialbusiness,
            ) {
                let key = SummationKey {
                    tract_id: get_tract(fcc.block_geoid),
                    product_type: product_type.clone(),
                };

                if !summarization.contains_key(&key) {
                    summarization.insert(
                        key,
                        Summation {
                            total: 1,
                            residential: if is_consumer(cf_residentialbusiness) {
                                1
                            } else {
                                0
                            },
                        },
                    );
                } else if let Some(existing) = summarization.get(&key) {
                    summarization.insert(
                        key,
                        Summation {
                            total: existing.total + 1,
                            residential: if is_consumer(cf_residentialbusiness) {
                                existing.residential + 1
                            } else {
                                existing.residential
                            },
                        },
                    );
                }
            }
        }
    }

    summarization
}
