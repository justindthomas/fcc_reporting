use std::{collections::HashMap, io};

use crate::{
    analysis::{Summation, SummationKey},
    emerald::ProductType,
};

pub fn broadband_subscription_report(summarization: &HashMap<SummationKey, Summation>) {
    let mut wtr = csv::WriterBuilder::new()
        .from_path("output/reports/foo.csv")
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
        if let ProductType::Internet(product) = key.product_type {
            wtr.write_record(&[
                key.tract_id.clone(),
                product.0.to_string(),
                product.1.to_string(),
                product.2.to_string(),
                summation.total.to_string(),
                summation.residential.to_string(),
            ])
            .ok();
        }
    }

    wtr.flush().ok();
}
