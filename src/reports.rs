use std::collections::{HashMap, HashSet};

use crate::{
    analysis::{LocationSummationKey, Summation, TractSummationKey},
    emerald::ProductType,
    routes::focus::{BroadbandStatistic, VoipStatistic},
};

pub fn broadband_subscription_report(
    uuid: &str,
    summarization: &HashMap<TractSummationKey, Summation>,
) -> BroadbandStatistic {
    let mut statistic = BroadbandStatistic::default();

    let now = chrono::Utc::now().timestamp();
    let mut wtr = csv::WriterBuilder::new()
        .from_path(format!(
            "output/reports/broadband_subscription-{now}-{uuid}.csv"
        ))
        .unwrap();

    wtr.write_record([
        "tract",
        "technology_code",
        "advertised_download_speed",
        "advertised_upload_speed",
        "total_connections",
        "consumer_connections",
    ])
    .ok();

    for (key, summation) in summarization {
        statistic = statistic + ((*key).clone(), (*summation).clone()).into();
        if let ProductType::Internet(service_profile) = key.product_type.clone() {
            wtr.write_record(&[
                key.tract_id.clone(),
                service_profile.technology_code.to_string(),
                service_profile.committed_bandwidth_down.to_string(),
                service_profile.committed_bandwidth_up.to_string(),
                summation.total.to_string(),
                summation.residential.to_string(),
            ])
            .ok();
        }
    }

    wtr.flush().ok();

    statistic
}

pub fn voice_subscription_report(
    uuid: &String,
    summarization: &HashMap<TractSummationKey, Summation>,
) -> VoipStatistic {
    let mut statistic = VoipStatistic::default();
    let now = chrono::Utc::now().timestamp();
    let mut wtr = csv::WriterBuilder::new()
        .from_path(format!(
            "output/reports/voice_subscription-{now}-{uuid}.csv"
        ))
        .unwrap();

    wtr.write_record([
        "tract",
        "service_type",
        "total_lines_or_subscriptions",
        "consumer_lines_or_subscriptions",
    ])
    .ok();

    for (key, summation) in summarization {
        statistic = statistic + ((*key).clone(), (*summation).clone()).into();
        if let ProductType::Voip = key.product_type {
            wtr.write_record(&[
                key.tract_id.clone(),
                "1".to_string(),
                summation.total.to_string(),
                summation.residential.to_string(),
            ])
            .ok();
        }
    }

    wtr.flush().ok();

    statistic
}

pub fn broadband_availability_report(uuid: &String, summarization: &HashSet<LocationSummationKey>) {
    let now = chrono::Utc::now().timestamp();
    let mut wtr = csv::WriterBuilder::new()
        .from_path(format!(
            "output/reports/broadband_availability-{now}-{uuid}.csv"
        ))
        .unwrap();

    wtr.write_record([
        "provider_id",
        "brand_name",
        "location_id",
        "technology",
        "max_advertised_download_speed",
        "max_advertised_upload_speed",
        "low_latency",
        "business_residential_code",
    ])
    .ok();

    for key in summarization {
        if let ProductType::Internet(service_profile) = key.product_type.clone() {
            wtr.write_record(&[
                "410035".to_string(),
                "Emerald Broadband, LLC".to_string(),
                key.location_id.clone(),
                service_profile.technology_code.to_string(),
                service_profile.available_bandwidth_down.to_string(),
                service_profile.available_bandwidth_up.to_string(),
                "1".to_string(),
                "X".to_string(),
            ])
            .ok();
        }
    }

    wtr.flush().ok();
}
