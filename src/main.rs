mod models;
mod web;
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

static CONF: Lazy<Config> = Lazy::new(|| {
    let mut settings = Config::default();
    settings
        .merge(config::File::with_name(".conf"))
        .unwrap()
        .clone()
});

static DB: Lazy<Db> = Lazy::new(|| sled::open("db").expect("Can't open the DB"));


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut interval_day = time::interval(Duration::from_secs(5*60));
    loop {
        interval_day.tick().await;
        let shops = CONF.get::<Vec<String>>("ebay.shops").unwrap();

        for shop_name in &shops {

            let mut web = Web::new(shop_name).await?; // вынести за пределы цикла

            let mut orders: HashSet<String> = if let Ok(Some(x)) = DB.get("orders") {
                serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
            } else {
                HashSet::new()
            };
            let mut new_orders_found = false;

            let api_endpoint = "/developer/analytics/v1_beta/rate_limit/?api_name=Analytics";
        
            for i in 1..=3 {        // перенести эту проверку внутрь get
                print!("{} - checking connection.. ", shop_name);
                let reply = web.get(api_endpoint)
                    .await?
                    .text()
                    .await?;
                if !reply.contains("errorId") {
                    println!("OK");
                    break
                } else {
                    println!("Error: {}", reply);
                    match i {
                        1 => web.refresh().await?,
                        2 => web.auth(shop_name).await?,
                        _ => println!("Error during token exchagne cycle"),
                    }
                }
            }
            // we should have a token at that point

            let api_endpoint = "/sell/fulfillment/v1/order?filter=orderfulfillmentstatus:%7BNOT_STARTED%7CIN_PROGRESS%7D";

            let reply = web.get(api_endpoint)
            .await?
            .text()
            .await?;

            let deserializer = &mut serde_json::Deserializer::from_str(&reply);
            let json: EbayOrders = match serde_path_to_error::deserialize(deserializer) {
                Ok(x) => x,
                Err(e) =>  {
                        println!("Deserealisation error: {}\n\nEbay Orders: {}", e, reply);
                        continue;
                    },
            };
            
            // {
            //     println!("{}", err);
            //     continue;
            // };

            for order in json.orders {
                let total: f64 = order.pricing_summary.total.value.clone().parse().unwrap_or_default();
                if total > CONF.get::<f64>("ebay.sale_to_notify").unwrap() {
                    let order_id = order.order_id;

                    if !orders.contains(&order_id) {
                        new_orders_found = true;
                        println!("New order found: {} £{}", order_id, total);
                        orders.insert(order_id);

                        if CONF.get::<bool>("send_messages").unwrap() {
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

            if new_orders_found && CONF.get::<bool>("send_messages").unwrap() {
                DB.insert("orders", serde_json::to_string(&orders).unwrap().as_bytes()).unwrap();
            }

        }
        DB.flush()?;

    }

    // Ok(())
}



