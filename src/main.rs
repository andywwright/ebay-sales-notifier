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
use thiserror::Error;
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

#[derive(Error, Debug)]
pub enum LocalError {
    #[error("Fagent - Error during token exchagne cycle")]
    FagentTokenError,
    #[error("FreeAgent server returned unknown error: `{0}`")]
    FagentUnknownError(String),
    #[error("Ebay - Error during token exchagne cycle")]
    EbayTokenError,
    #[error("eBay server message: 'System error'")]
    EbaySystemError,
    #[error("eBay server returned unknown error: `{0}`")]
    EbayUnknownError(String),
    #[error("Invalid item number or invalid transaction or feedback already left")]
    EbayFeedbackAlreadyLeft,
    #[error("eBay server returned unknown error: `{0}`")]
    EbayFeedbackUnknownError(String),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let debug = CONF.get::<bool>("debug")?;
    if debug {
        println!("Running in DEBUG mode");
        let api_endpoint = "/ws/api.dll";
        let shop_name = "mobriver";
        let mut ebay_api = EbayApi::new(&shop_name).await?;

        // first call

        let call_name = "SetNotificationPreferences";
        let body = format!(
            r#"
            <?xml version="1.0" encoding="utf-8"?>
            <SetNotificationPreferencesRequest xmlns="urn:ebay:apis:eBLBaseComponents">
              <ApplicationDeliveryPreferences>
                <AlertEmail>mailto://andy4usa@gmail.com</AlertEmail>
                <AlertEnable>Enable</AlertEnable>
                <ApplicationEnable>Enable</ApplicationEnable>
                <ApplicationURL>https://mobriver.co.uk</ApplicationURL>
                <DeviceType>Platform</DeviceType>
              </ApplicationDeliveryPreferences>
              <UserDeliveryPreferenceArray>
                <NotificationEnable>
                  <EventType>Feedback</EventType>
                  <EventEnable>Enable</EventEnable>
                </NotificationEnable>
                <NotificationEnable>
                  <EventType>AuctionCheckoutComplete</EventType>
                  <EventEnable>Enable</EventEnable>
                </NotificationEnable>
              </UserDeliveryPreferenceArray>
            </SetNotificationPreferencesRequest>
            "#
        );
        let reply = ebay_api.post(api_endpoint, call_name, body).await?;

        println!("\nThe first reply: {reply}\n");

        // second call

        let call_name = "GetNotificationPreferences";
        let body = format!(
            r#"
                <?xml version="1.0" encoding="utf-8"?>
                <GetNotificationPreferencesRequest xmlns="urn:ebay:apis:eBLBaseComponents">
                    <PreferenceLevel>User</PreferenceLevel>
                </GetNotificationPreferencesRequest>
            "#
        );
        let reply = ebay_api.post(api_endpoint, call_name, body).await?;

        // create_bank_transactions().await?;

        println!("The second reply: {reply} \nExiting... OK");
        return Ok(());
    }
    println!("+");
    let write_orders_and_send_messages = CONF.get::<bool>("send_messages")?;
    let mut interval_5_min = time::interval(Duration::from_secs(5 * 60));
    let shops = CONF.get::<HashSet<String>>("ebay.shops")?;
    let shops_for_feedback = CONF.get::<HashSet<String>>("shops_for_feedback")?;
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
    let mut ping_timer = 0;
    loop {
        interval_5_min.tick().await;

        if i == 0 {
            feedback::leave().await?;
            for shop_name in &shops_for_refresh {
                let mut ebay_api = EbayApi::new(shop_name).await?;
                if let Err(e) = ebay_api.refresh_access_token(false).await {
                    println!("{shop_name} - token refreshing has failed - {e}");
                }
            }
        }
        i += 1;
        if i == 20 {
            i = 0
        }

        ping_timer += 1;
        if ping_timer == 100 {
            println!("+");
            ping_timer = 0;
        }

        for shop_name in &shops {
            let mut ebay_api = EbayApi::new(shop_name).await?;

            let mut orders: HashSet<String> = if let Ok(Some(x)) = DB.get("orders") {
                serde_json::from_str(std::str::from_utf8(&x)?)?
            } else {
                HashSet::new()
            };
            let mut new_orders_found = false;

            let api_endpoint = "/sell/fulfillment/v1/order?filter=orderfulfillmentstatus:%7BNOT_STARTED%7CIN_PROGRESS%7D";

            let reply = match ebay_api.get(api_endpoint).await {
                // найти как развернуть ошибку без матч
                Ok(x) => x,
                Err(e) => {
                    println!("{shop_name} - orders processing has failed - {e}");
                    continue;
                }
            };

            let deserializer = &mut serde_json::Deserializer::from_str(&reply);
            let json: EbayOrders = match serde_path_to_error::deserialize(deserializer) {
                Ok(x) => x,
                Err(e) => {
                    println!("{shop_name} - orders processing has failed - deserealisation error: {e}\n\nReply body: ###{reply}###\n");
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
                if total > CONF.get::<f64>("ebay.sale_to_notify")? {
                    let order_id = order.order_id;

                    if !orders.contains(&order_id) {
                        new_orders_found = true;
                        let item = &order.line_items[0].title;
                        let msg = format!("£{total} - {shop_name} - {item}");
                        println!("{msg}");
                        orders.insert(order_id);

                        if write_orders_and_send_messages {
                            let url = format!("https://api.telegram.org/bot863650897:AAE-usx-Av7yk0C1csClrS-nFLgDzVTrNmo/sendMessage?chat_id=-1001451097938&text={msg}");
                            if let Err(e) = reqwest::get(url).await?.text().await {
                                println!(
                                    "{shop_name} - sending a message to telegram has failed - {e}"
                                );
                            }
                        }
                    }
                }
            }
            if new_orders_found && write_orders_and_send_messages {
                DB.insert("orders", serde_json::to_string(&orders)?.as_bytes())?;
            }
        }
        DB.flush()?;
    }
    // Ok(())
}
