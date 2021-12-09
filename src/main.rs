mod models;
mod web;
mod feedback;
use models::*;
use web::*;

use reqwest;
use std::collections::HashMap;
use std::process::Command;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RefreshToken, Scope, TokenResponse, TokenUrl,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;
use once_cell::sync::Lazy;
use config::Config;
use sled::Db;
use chrono::prelude::*;
use tokio::time;
use std::time::Duration;
use std::collections::HashSet;
// use serde_derive::{Serialize, Deserialize};

static CONF: Lazy<Config> = Lazy::new(|| {
    let mut settings = Config::default();
    settings
        .merge(config::File::with_name(".conf"))
        .unwrap()
        .clone()
});

static DB: Lazy<Db> = Lazy::new(|| sled::open("db").expect("Can't open the DB"));

pub const API_URL: &str = "https://api.ebay.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let send_messages = CONF.get::<bool>("send_messages").unwrap();
    let mut interval_day = time::interval(Duration::from_secs(5*60));
    let mut i = 0;
    loop {
        interval_day.tick().await;

        if i == 0 { feedback::leave().await? }
        i += 1;
        if i == 20 { i = 0 }

        let shops = CONF.get::<Vec<String>>("ebay.shops").unwrap();

        for shop_name in &shops {

            let mut web = Web::new(shop_name).await?;

            let mut orders: HashSet<String> = if let Ok(Some(x)) = DB.get("orders") {
                serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
            } else {
                HashSet::new()
            };
            let mut new_orders_found = false;

            let api_endpoint = "/sell/fulfillment/v1/order?filter=orderfulfillmentstatus:%7BNOT_STARTED%7CIN_PROGRESS%7D";

            let reply = web.get(api_endpoint).await?;

            let deserializer = &mut serde_json::Deserializer::from_str(&reply);
            let json: EbayOrders = match serde_path_to_error::deserialize(deserializer) {
                Ok(json) => json,
                Err(e) =>  {
                        println!("Deserealisation error: {}\n\nEbay Orders: {}", e, reply);
                        continue;
                    },
            };

            for order in json.orders {
                let total: f64 = order.pricing_summary.total.value.clone().parse().unwrap_or_default();
                if total > CONF.get::<f64>("ebay.sale_to_notify").unwrap() {
                    let order_id = order.order_id;

                    if !orders.contains(&order_id) {
                        new_orders_found = true;
                        println!("New order found: {} £{}", order_id, total);
                        orders.insert(order_id);

                        if send_messages {
                            let bot_url = "https://api.telegram.org/bot863650897:AAE-usx-Av7yk0C1csClrS-nFLgDzVTrNmo/sendMessage?chat_id=-1001451097938&text=";
                            let url = format!("{}£{} from {} for {}", bot_url, total, shop_name, order.line_items[0].title);
                            reqwest::get(url)
                                .await?
                                .text()
                                .await?;
                        }
                    }
                }
            }
            if new_orders_found && send_messages {
                DB.insert("orders", serde_json::to_string(&orders).unwrap().as_bytes()).unwrap();
            }
        }
        DB.flush()?;
    }
    // Ok(())
}


