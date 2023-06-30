#[macro_use]
extern crate rocket;

extern crate log;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

pub mod analysis;
pub mod emerald;
pub mod fcc;
pub mod reports;
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
