#[macro_use]
extern crate rocket;

extern crate log;

use core::time;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, thread};

pub mod routes;

pub enum ProductType {
    Internet((u8, u16, u16)),
    Voip,
    Fax,
    Admin,
}

lazy_static! {
    pub static ref API_KEY: String = {
        dotenv().ok();
        env::var("API_KEY").expect("API_KEY must be set")
    };
    pub static ref API_URL: String = {
        dotenv().ok();
        env::var("API_URL").expect("API_URL must be set")
    };
    pub static ref PRODUCT_CODES: HashMap<&'static str, ProductType> = HashMap::from([
        ("fttp1000", ProductType::Internet((50, 1000, 1000))),
        ("fttp800", ProductType::Internet((50, 800, 800))),
        ("fttp400", ProductType::Internet((50, 400, 400))),
        ("fttp250", ProductType::Internet((50, 250, 250))),
        ("fttp100", ProductType::Internet((50, 100, 100))),
        ("fttp25", ProductType::Internet((50, 25, 25))),
        ("fw25", ProductType::Internet((70, 25, 25))),
        ("fw50", ProductType::Internet((70, 50, 50))),
        ("fw75", ProductType::Internet((70, 75, 75))),
        ("fw100", ProductType::Internet((70, 100, 100))),
        ("ens1g", ProductType::Internet((50, 1000, 1000))),
        ("enscustom", ProductType::Internet((50, 1000, 1000))),
        ("ens100mbps", ProductType::Internet((50, 100, 100))),
        ("gf100", ProductType::Internet((10, 100, 100))),
        ("voipfax", ProductType::Voip),
        ("voippbxr", ProductType::Voip),
        ("voiprpxr", ProductType::Voip),
        ("voipbus", ProductType::Voip),
        ("fax2email", ProductType::Fax),
        ("installation-quote", ProductType::Admin),
        ("paymentagreement", ProductType::Admin),
        ("pre-reg", ProductType::Admin),
        ("service-call-quote", ProductType::Admin),
        ("acp", ProductType::Admin),
        ("ipv427", ProductType::Admin),
        ("ipv428", ProductType::Admin),
        ("ipv429", ProductType::Admin),
        ("ipstatic", ProductType::Admin),
    ]);
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionObject {
    EventBasedAddon,
    ChargedEventBasedAddon,
    Coupon,
    Subscription,
    BillingAddress,
    PaymentMethod,
    Card,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionEvent {
    SubscriptionActivation,
    PlanActivation,
    ContractTermination,
    SubscriptionCreation,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventBasedAddon {
    id: String,
    quantity: u32,
    unit_price: u32,
    on_event: SubscriptionEvent,
    charge_once: bool,
    object: SubscriptionObject,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChargedEventBasedAddon {
    id: String,
    last_charged_at: u64,
    object: SubscriptionObject,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Coupon {
    coupon_id: String,
    applied_count: u8,
    object: SubscriptionObject,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BillingAddress {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    line1: Option<String>,
    city: Option<String>,
    state_code: Option<String>,
    state: Option<String>,
    country: Option<String>,
    zip: Option<String>,
    validation_status: Option<String>,
    object: SubscriptionObject,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethod {
    object: SubscriptionObject,
    #[serde(rename = "type")]
    kind: Option<String>,
    reference_id: Option<String>,
    gateway: Option<String>,
    gateway_account_id: Option<String>,
    status: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    auto_collection: Option<String>,
    net_term_days: Option<u8>,
    allow_direct_debit: Option<bool>,
    created_at: Option<u64>,
    taxability: Option<String>,
    updated_at: Option<u64>,
    locale: Option<String>,
    pii_cleared: Option<String>,
    channel: Option<String>,
    resource_version: Option<u64>,
    deleted: Option<bool>,
    object: Option<String>,
    billing_address: BillingAddress,
    card_status: Option<String>,
    promotional_credits: Option<u32>,
    refundable_credits: Option<u32>,
    excess_payments: Option<u32>,
    unbilled_charges: Option<u32>,
    preferred_currency_code: Option<String>,
    primary_payment_source_id: Option<String>,
    payment_method: Option<PaymentMethod>,
    tax_providers_fields: Option<Vec<String>>,
    cf_residentialbusiness: Option<String>,
    cf_service_address: Option<String>,
    cf_service_city_st_zip: Option<String>,
    cf_census_block_no: Option<String>,
    cf_paper_billing: Option<String>,
    cf_acp: Option<String>,
    consolidated_invoicing: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    status: Option<String>,
    gateway: Option<String>,
    gateway_account_id: String,
    first_name: Option<String>,
    last_name: Option<String>,
    iin: Option<String>,
    last4: Option<String>,
    card_type: Option<String>,
    funding_type: Option<String>,
    expiry_month: Option<u8>,
    expiry_year: Option<u32>,
    created_at: Option<u64>,
    updated_at: Option<u64>,
    resource_version: Option<u64>,
    object: SubscriptionObject,
    masked_number: Option<String>,
    customer_id: Option<String>,
    payment_source_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subscription {
    id: Option<String>,
    plan_id: Option<String>,
    plan_quantity: Option<u8>,
    plan_unit_price: Option<u32>,
    billing_period: Option<u8>,
    billing_period_unit: Option<String>,
    customer_id: Option<String>,
    plan_amount: Option<u32>,
    plan_free_quantity: Option<u32>,
    status: Option<String>,
    current_term_start: Option<u64>,
    current_term_end: Option<u64>,
    next_billing_at: Option<u64>,
    created_at: Option<u64>,
    started_at: Option<u64>,
    activated_at: Option<u64>,
    updated_at: Option<u64>,
    has_scheduled_changed: Option<bool>,
    channel: Option<String>,
    resource_version: Option<u64>,
    deleted: Option<bool>,
    object: SubscriptionObject,
    coupon: Option<String>,
    currency_code: Option<String>,
    event_based_addons: Option<Vec<EventBasedAddon>>,
    charged_event_based_addons: Option<Vec<ChargedEventBasedAddon>>,
    coupons: Option<Vec<Coupon>>,
    due_invoices_count: Option<u8>,
    mrr: Option<u32>,
    exchange_rate: Option<f32>,
    base_currency_code: Option<String>,
    has_scheduled_advance_invoices: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SubscriptionApiItem {
    subscription: Subscription,
    customer: Customer,
    card: Option<Card>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiResponse {
    list: Vec<SubscriptionApiItem>,
    next_offset: Option<String>,
}

pub fn retrieve_subscriptions() -> Vec<SubscriptionApiItem> {
    let mut subscriptions: Vec<SubscriptionApiItem> = vec![];
    let mut offset: Option<String> = {
        let mut page = retrieve_subscription_page(None);
        subscriptions.append(&mut page.list);
        page.next_offset
    };

    while offset.is_some() {
        thread::sleep(time::Duration::from_millis(1000));
        println!("RETRIEVING PAGE: {offset:#?}");
        let mut page = retrieve_subscription_page(offset);
        subscriptions.append(&mut page.list);
        offset = page.next_offset;
    }

    subscriptions
}

pub fn retrieve_subscription_page(offset: Option<String>) -> ApiResponse {
    let client = reqwest::blocking::Client::new();

    let offset = {
        if let Some(offset) = offset {
            format!("&offset={offset}")
        } else {
            "".to_string()
        }
    };

    client
        .get(format!(
            "{}?limit=100&status[is]=active{offset}",
            &*crate::API_URL
        ))
        .basic_auth(&*crate::API_KEY, None::<String>)
        .send()
        .unwrap()
        .json::<ApiResponse>()
        .unwrap()
}
