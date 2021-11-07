use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use dotenv::dotenv;
use log::{error, info};
use std::error::Error;
use std::time::Duration;
use std::{env, thread};

use crate::price_manager::db_price_manager::db_price_manager::DBPriceManager;
use crate::price_manager::price_manager::PriceManager;

use crate::price_db::mongo_price_db::mongo_price_db::{MongoPriceDB, MongoPriceDBConfig};
use kraken_rest_api::api::api::KrakenAPI;

pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("api_key")
                .short("k")
                .long("api_key")
                .value_name("API_KEY")
                .help("Set API_KEY. Can also set with env variable: API_KEY. This commandline argument take precedence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("api_secret")
                .short("s")
                .long("api_secret")
                .value_name("API_SECRET")
                .help("Set API_SECRET. Can also set with env variable: API_SECRET. This commandline argument take precedence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("db_username")
                .short("u")
                .long("db_username")
                .value_name("DB_USERNAME")
                .help("Set DB_USERNAME. Can also set with env variable: DB_USERNAME. This commandline argument take precedence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("db_password")
                .short("p")
                .long("db_password")
                .value_name("DB_PASSWORD")
                .help("Set DB_PASSWORD. Can also set with env variable: DB_PASSWORD. This commandline argument take precedence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("db_host")
                .short("h")
                .long("db_host")
                .value_name("DB_HOST")
                .help("Set DB_HOST. Can also set with env variable: DB_HOST. This commandline argument take precedence")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("db_name")
                .short("n")
                .long("db_name")
                .value_name("DB_NAME")
                .help("Set DB_NAME. Can also set with env variable: DB_NAME. This commandline argument take precedence")
                .takes_value(true),
        )
        .get_matches();

    env_logger::init();

    let api_key = matches
        .value_of("api_key")
        .map(|s| s.to_owned())
        .or(env::var("API_KEY").ok());

    let api_secret = matches
        .value_of("api_secret")
        .map(|s| s.to_owned())
        .or(env::var("API_SECRET").ok());

    let db_username = matches
        .value_of("db_username")
        .map(|s| s.to_owned())
        .or(env::var("DB_USERNAME").ok());

    let db_password = matches
        .value_of("db_password")
        .map(|s| s.to_owned())
        .or(env::var("DB_PASSWORD").ok());

    let db_host = matches
        .value_of("db_host")
        .map(|s| s.to_owned())
        .or(env::var("DB_HOST").ok());

    let db_name = matches
        .value_of("db_name")
        .map(|s| s.to_owned())
        .or(env::var("DB_NAME").ok());

    let db = MongoPriceDB::new(MongoPriceDBConfig::new(
        &db_username.expect("DB_USERNAME not set"),
        &db_password.expect("DB_PASSWORD not set"),
        &db_host.expect("DB_HOST not set"),
        &db_name.expect("DB_NAME not set"),
    ))
    .await?;
    let kraken_api = KrakenAPI::new(
        (&api_key.expect("API_KEY not set")).to_string(),
        (&api_secret.expect("API_SECRET not set")).to_string(),
    );

    let manager = DBPriceManager::new(Box::new(db), kraken_api);
    sync_price_data(Box::new(manager)).await?;
    Ok(())
}

pub async fn sync_price_data(manager: Box<dyn PriceManager>) -> Result<(), Box<dyn Error>> {
    loop {
        match manager.update_price_data().await {
            Ok(()) => info!("ok"),
            Err(e) => error!("error: {:?}", e),
        }
        thread::sleep(Duration::new(60, 0));
    }
}
