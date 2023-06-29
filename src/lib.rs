#[macro_use]
extern crate rocket;

extern crate log;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;

pub mod routes;

lazy_static! {
    pub static ref API_KEY: String = {
        dotenv().ok();
        env::var("API_KEY").expect("API_KEY must be set")
    };
    pub static ref API_URL: String = {
        dotenv().ok();
        env::var("API_URL").expect("API_URL must be set")
    };
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventBasedAddon {
    id: String,
    quantity: u16,
    unit_price: u16,
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
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    line1: String,
    city: String,
    state_code: String,
    state: String,
    country: String,
    zip: String,
    validation_status: String,
    object: SubscriptionObject,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaymentMethod {
    object: SubscriptionObject,
    #[serde(rename = "type")]
    kind: String,
    reference_id: String,
    gateway: String,
    gateway_account_id: String,
    status: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
    auto_collection: String,
    net_term_days: u8,
    allow_direct_debit: bool,
    created_at: u64,
    taxability: String,
    updated_at: u64,
    locale: String,
    pii_cleared: String,
    channel: String,
    resource_version: u64,
    deleted: bool,
    object: String,
    billing_address: BillingAddress,
    card_status: String,
    promotional_credits: u16,
    refundable_credits: u16,
    excess_payments: u16,
    unbilled_charges: u16,
    preferred_currency_code: String,
    primary_payment_source_id: Option<String>,
    payment_method: Option<PaymentMethod>,
    tax_providers_fields: Vec<String>,
    cf_residentialbusiness: String,
    cf_service_address: String,
    cf_service_city_st_zip: String,
    cf_census_block_no: String,
    cf_paper_billing: String,
    cf_acp: String,
    consolidated_invoicing: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Card {
    status: String,
    gateway: String,
    gateway_account_id: String,
    first_name: String,
    last_name: String,
    iin: String,
    last4: String,
    card_type: String,
    funding_type: String,
    expiry_month: u8,
    expiry_year: u16,
    created_at: u64,
    updated_at: u64,
    resource_version: u64,
    object: SubscriptionObject,
    masked_number: String,
    customer_id: String,
    payment_source_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subscription {
    id: String,
    plan_id: String,
    plan_quantity: u8,
    plan_unit_price: u16,
    billing_period: u8,
    billing_period_unit: String,
    customer_id: String,
    plan_amount: u16,
    plan_free_quantity: u16,
    status: String,
    current_term_start: u64,
    current_term_end: u64,
    next_billing_at: u64,
    created_at: u64,
    started_at: u64,
    activated_at: u64,
    updated_at: u64,
    has_scheduled_changed: Option<bool>,
    channel: String,
    resource_version: u64,
    deleted: bool,
    object: SubscriptionObject,
    coupon: String,
    currency_code: String,
    event_based_addons: Vec<EventBasedAddon>,
    charged_event_based_addons: Vec<ChargedEventBasedAddon>,
    coupons: Vec<Coupon>,
    due_invoices_count: u8,
    mrr: u16,
    exchange_rate: f32,
    base_currency_code: String,
    has_scheduled_advance_invoices: bool,
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

pub fn retrieve_subscriptions() {
    let client = reqwest::blocking::Client::new();
    let body = client
        .get(&*crate::API_URL)
        .basic_auth(&*crate::API_KEY, None::<String>)
        .send()
        .unwrap()
        .json::<ApiResponse>()
        .unwrap();

    println!("{body:#?}");
    println!("items: {}", body.list.len());
}
