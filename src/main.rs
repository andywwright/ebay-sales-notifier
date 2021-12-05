mod models;
use models::*;

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

static EBAY_CLIENT_ID: Lazy<String> = Lazy::new(|| CONF.get::<String>("ebay.client_id").unwrap());
static EBAY_CLIENT_SECRET: Lazy<String> = Lazy::new(|| CONF.get::<String>("ebay.client_secret").unwrap());
// static EBAY_URL_SCHEME: Lazy<String> = Lazy::new(|| CONF.get::<String>("ebay.ru_name").unwrap());

static EBAY_SCOPE: Lazy<Scope> =
    Lazy::new(|| Scope::new("https://api.ebay.com/oauth/api_scope/sell.fulfillment https://api.ebay.com/oauth/api_scope".to_string()));

pub const API_URL: &str = "https://api.ebay.com";

static DB: Lazy<Db> = Lazy::new(|| sled::open("db").expect("Can't open the DB"));


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> { // ПЕРЕНЕСТИ ПРОВЕРКУ ВНУТРЬ ГЕТ!!!

    let mut interval_day = time::interval(Duration::from_secs(5*60));
    loop {
        interval_day.tick().await;
        let shops = CONF.get::<Vec<String>>("ebay.shops").unwrap();

        for shop_name in &shops {

            let mut orders: HashSet<String> = if let Ok(Some(x)) = DB.get("orders") {
                serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
            } else {
                HashSet::new()
            };
            let mut new_orders_found = false;


            let key = ["oauth_token_ebay_", shop_name].concat();
            let tokens: Tokens = if let Ok(Some(x)) = DB.get(&key) {
                serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
            } else {
                println!("Getting a eBay user permission for {}", shop_name);
                auth(shop_name).await?;
                DB.flush()?;
                panic!()
                // auth(shop_name).unwrap()
            };
            // let access_token = refresh(token.refresh_token.clone()).unwrap();
            let mut token = tokens.access_token;
            let api_endpoint = "/developer/analytics/v1_beta/rate_limit/?api_name=Analytics";
        
            for i in 1..=3 {        // перенести эту проверку внутрь get
                print!("{} - checking connection.. ", shop_name);
                let reply = get(api_endpoint, &token)
                    .await?
                    .text()
                    .await?;
                if !reply.contains("errorId") {
                    println!("OK");
                    break
                } else {
                    println!("Error: {}", reply);
                    match i {
                        1 => token = refresh(shop_name, &tokens.refresh_token).await?,
                        2 => token = auth(shop_name).await?,
                        _ => println!("Error during token exchagne cycle"),
                    }
                }
            }
            // we should have a token at that point

            let api_endpoint = "/sell/fulfillment/v1/order?filter=orderfulfillmentstatus:%7BNOT_STARTED%7CIN_PROGRESS%7D";

            let reply = get(api_endpoint, &token)
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

            if new_orders_found {
                DB.insert("orders", serde_json::to_string(&orders).unwrap().as_bytes()).unwrap();
            }

        }
        DB.flush()?;

    }

    // Ok(())
}

async fn get(api_endpoint: &str, token: &str) -> Result<reqwest::Response, reqwest::Error> {
    let mut params: HashMap<&str, String> = HashMap::new();
    params.insert(
        "limit",
        CONF.get::<String>("ebay.limit").unwrap().to_string(),
    );
    let url = [API_URL, api_endpoint].concat();
    let client = reqwest::Client::new();
    client
        .get(url)
        .query(&params)
        .header("Authorization", ["Bearer ", token].concat())
        .header("Content-Type", "application/json")
        .send()
        .await
}


async fn auth(shop_name:&str) -> Result<String, Box<dyn std::error::Error>> {

    let mut acces_token = String::new();
    
    let auth_url = AuthUrl::new("https://auth.ebay.com/oauth2/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://api.ebay.com/identity/v1/oauth2/token".to_string())
        .expect("Invalid token endpoint URL");        

    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(
        ClientId::new(EBAY_CLIENT_ID.clone()),
        Some(ClientSecret::new(EBAY_CLIENT_SECRET.clone())),
        auth_url,
        Some(token_url),
    );

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(|| CsrfToken::new("http://localhost:8080".to_string()))
        .add_scope((*EBAY_SCOPE).clone())
        .add_extra_param("redirect_uri", "Andrey_Soludano-AndreySo-crm-PR-eemypgxor")
        .url();



    let url = format!("start {}", authorize_url.to_string()).replace("&", "^&");

    let output = Command::new("cmd.exe").args(["/C", &url]).output();

    if let Err(e) = output {
        println!("We couldn't start your browser: {:?}\n\n
        Please start it manually and open this URL:\n{}\n", e, authorize_url.to_string());
    }

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    loop {
        if let Ok((mut stream, _)) = listener.accept().await {
            let code;
            let state;
            {
                let mut reader = BufReader::new(&mut stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).await.unwrap();

                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                let code_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "code"
                    })
                    .unwrap();

                let (_, value) = code_pair;
                code = AuthorizationCode::new(value.into_owned());

                let state_pair = url
                    .query_pairs()
                    .find(|pair| {
                        let &(ref key, _) = pair;
                        key == "state"
                    })
                    .unwrap();

                let (_, value) = state_pair;
                state = CsrfToken::new(value.into_owned());
            }

            let message = "Go back to your terminal :)";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).await.unwrap();

            println!("Request returned the following code:\n{}\n", code.secret());
            println!(
                "Request returned the following state:\n{} (expected `{}`)\n",
                state.secret(),
                csrf_state.secret()
            );

            // Exchange the code with a token.
            let token_res = client
                .exchange_code(code)
                .add_extra_param("redirect_uri", "Andrey_Soludano-AndreySo-crm-PR-eemypgxor")
                .request_async(async_http_client)
                .await;

            println!("Github returned the following token:\n{:?}\n", token_res);

            if let Ok(token) = token_res {
                // обрабатывать если вернул ошибку!!!
                let tokens = Tokens::new(
                    token.access_token().secret().clone(),
                    token.expires_in().unwrap().as_secs(),
                    token.refresh_token().unwrap().secret().clone(),
                );
                dbg!(&tokens);

                let key = ["oauth_token_ebay_", shop_name].concat();
                DB.insert(&key, serde_json::to_string(&tokens).unwrap().as_bytes())
                    .unwrap();
                acces_token = token.access_token().secret().clone();

            }
            break;
        }
    }
    Ok(acces_token)
}

async fn refresh(shop_name: &str, refresh_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let auth_url = AuthUrl::new("https://auth.ebay.com/oauth2/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://api.ebay.com/identity/v1/oauth2/token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(ClientId::new(
        EBAY_CLIENT_ID.clone()), 
Some(ClientSecret::new(EBAY_CLIENT_SECRET.clone())),
        auth_url, Some(token_url));

    let res = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
        .add_scope((*EBAY_SCOPE).clone())
        .request_async(async_http_client)
        .await;

    // dbg!(&res);

    // if let Ok(token) = res {
    // обрабатывать если вернул ошибку!!!
    let token = res.unwrap();

    let tokens = Tokens::new(
        token.access_token().secret().clone(),
        token.expires_in().unwrap().as_secs(),
        refresh_token.to_string(),
    );

    let key = ["oauth_token_ebay_", shop_name].concat();
    DB.insert(&key, serde_json::to_string(&tokens).unwrap().as_bytes())
        .unwrap();

    Ok(token.access_token().secret().clone())
}

#[derive(Default, Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub client_id: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub token_type: String,
    pub user_id: String,
}

impl Tokens {
    pub fn new(access_token: String, expires_in: u64, refresh_token: String) -> Self {
        Tokens {
            access_token,
            expires_in,
            refresh_token,
            ..Default::default()
        }
    }
}
