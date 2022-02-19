mod ebay_api;
mod fagent_api;
mod feedback;
mod models;
use ebay_api::*;
use fagent_api::*;
use models::*;

use reqwest;
use std::collections::HashMap;
// use std::process::Command;
use chrono::prelude::*;
use config::Config;
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RefreshToken, Scope,
    TokenResponse, TokenUrl,
};
use once_cell::sync::Lazy;
use sled::Db;
use std::collections::HashSet;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::time;
use url::Url;
// use serde_derive::{Serialize, Deserialize};

static CONF: Lazy<Config> = Lazy::new(|| {
    let mut settings = Config::default();
    settings
        .merge(config::File::with_name(".conf"))
        .unwrap()
        .clone()
});

static DB: Lazy<Db> = Lazy::new(|| sled::open("db").expect("Can't open the DB"));

pub const EBAY_API_URL: &str = "https://api.ebay.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let debug = CONF.get::<bool>("debug").unwrap();
    if debug {
        // println!("Running in DEBUG mode");

        let mut fagent_api = FagentApi::new().await?;

        // let reply = fagent_api.get("categories").await?;

        let api_endpoint = format!("bank_transaction_explanations");

        let date = Utc::today().format("%Y-%m-%d");

        struct BankTransaction {
            account: i32,
            description: &'static str,
            category: &'static str,
            gross_value: i32,
        }

        let transactions = [
            BankTransaction {
                account: 1030038,
                description: "Amazon sales",
                category: "002",
                gross_value: 100,
            },
            BankTransaction {
                account: 1030038,
                description: "Amazon fees",
                category: "160",
                gross_value: -33,
            },
            BankTransaction {
                account: 1030344,
                description: "Ebay sales Mobriver",
                category: "003",
                gross_value: 100,
            },
            BankTransaction {
                account: 1030344,
                description: "Ebay fees",
                category: "161",
                gross_value: -33,
            },
            BankTransaction {
                account: 1030351,
                description: "Ebay sales Spasimira",
                category: "004",
                gross_value: 100,
            },
            BankTransaction {
                account: 1030351,
                description: "Ebay fees",
                category: "161",
                gross_value: -33,
            },
        ];

        for t in transactions {
            let body = format!(
                r#"
                {{ "bank_transaction_explanation":
                    {{
                        "bank_account":"https://api.freeagent.com/v2/bank_accounts/{}",
                        "dated_on":"{date}",
                        "description":"{}",
                        "category":"https://api.freeagent.com/v2/categories/{}",
                        "gross_value":"{}"
                    }}
                }}
            "#,
                t.account, t.description, t.category, t.gross_value
            );
            // let reply = fagent_api.post(&api_endpoint, body).await?;
            // println!("{reply}");
        }

        // return Ok(());

        // let reply = fagent_api.post(&api_endpoint, body).await?;

        // println!("{reply}");

        // println!("Exiting... OK");
        return Ok(());
    }

    let write_orders_and_send_messages = CONF.get::<bool>("send_messages").unwrap();
    let mut interval_5_min = time::interval(Duration::from_secs(5 * 60));
    let shops = CONF.get::<HashSet<String>>("ebay.shops").unwrap();
    let shops_for_feedback = CONF.get::<HashSet<String>>("shops_for_feedback").unwrap();
    let shops_for_refresh: HashSet<&String> = shops_for_feedback.union(&shops).collect();

    //     // spawn tasks that run in parallel
    // let tasks: Vec<_> = shops_for_refresh
    //     .iter()
    //     .map(|mut shop_name| {
    //         tokio::spawn(async {
    //             let mut web = Web::new(shop_name).await.unwrap();
    //             web.refresh(false)
    //         })
    //     })
    //     .collect();
    // // now await them to get the resolve's to complete
    // for task in tasks {
    //     task.await.unwrap();
    // }

    let mut i = 0;
    let mut print_timer = 0;
    loop {
        interval_5_min.tick().await;

        if i == 0 {
            feedback::leave().await?;
            for shop_name in &shops_for_refresh {
                let mut ebay_api = EbayApi::new(shop_name).await?;
                ebay_api.refresh_access_token(false).await?;
            }
        }
        i += 1;
        if i == 20 {
            i = 0
        }

        print_timer += 1;
        if print_timer == 100 {
            println!("+");
            print_timer = 0;
        }

        for shop_name in &shops {
            let mut ebay_api = EbayApi::new(shop_name).await?;

            let mut orders: HashSet<String> = if let Ok(Some(x)) = DB.get("orders") {
                serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
            } else {
                HashSet::new()
            };
            let mut new_orders_found = false;

            let api_endpoint = "/sell/fulfillment/v1/order?filter=orderfulfillmentstatus:%7BNOT_STARTED%7CIN_PROGRESS%7D";

            let reply = ebay_api.get(api_endpoint).await?;

            let deserializer = &mut serde_json::Deserializer::from_str(&reply);
            let json: EbayOrders = match serde_path_to_error::deserialize(deserializer) {
                Ok(json) => json,
                Err(e) => {
                    println!("Deserealisation error: {}\nReply body: ._{}_.", e, reply);
                    continue;
                }
            };

            for order in json.orders {
                let total: f64 = order
                    .pricing_summary
                    .total
                    .value
                    .clone()
                    .parse()
                    .unwrap_or_default();
                if total > CONF.get::<f64>("ebay.sale_to_notify").unwrap() {
                    let order_id = order.order_id;

                    if !orders.contains(&order_id) {
                        new_orders_found = true;
                        println!("New order found: {} £{}", order_id, total);
                        orders.insert(order_id);

                        if write_orders_and_send_messages {
                            let bot_url = "https://api.telegram.org/bot863650897:AAE-usx-Av7yk0C1csClrS-nFLgDzVTrNmo/sendMessage?chat_id=-1001451097938&text=";
                            let url = format!(
                                "{}£{} from {} for {}",
                                bot_url, total, shop_name, order.line_items[0].title
                            );
                            reqwest::get(url).await?.text().await?;
                        }
                    }
                }
            }
            if new_orders_found && write_orders_and_send_messages {
                DB.insert("orders", serde_json::to_string(&orders).unwrap().as_bytes())
                    .unwrap();
            }
        }
        DB.flush()?;
    }
    // Ok(())
}
