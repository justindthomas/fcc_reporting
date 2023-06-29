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
