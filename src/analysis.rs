use crate::{emerald::SubscriptionApiItem, fcc::FccRecord};
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
                        // isolate the initial emerald address numbers
                        emerald_address.clone().split_once(' '),
                        // isolate the city from the emerald complex string
                        emerald_city_state_zip.split_once(','),
                        // isolate the initial fcc address numbers
                        fcc_address.clone().split_once(' '),
                    ) {
                        // check if the city fields match
                        if emerald_city.to_uppercase() == fcc_city.to_uppercase()
                            // extract all the digits and compare (to minimize false positives in the fuzzy matching)
                            && digits(emerald_address.clone()) == digits(fcc_address.clone())
                            // check if the address numbers match exactly
                            && emerald_numbers == fcc_numbers
                            // check if the street names match roughly
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
