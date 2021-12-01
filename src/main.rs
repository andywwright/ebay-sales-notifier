// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // let url = "https://www.sageappliances.com/uk/en/parts-accessories/parts/sp0011389.html";
//     let url = "https://www.sageappliances.com/uk/en/parts-accessories/parts/sp0023675.html";
//     let resp = reqwest::get(url)
//         .await?
//         .text()
//         .await?;

//     if resp.contains("Sold out") {
//         println!("Sold out!!!")
//     } else {
//         println!("In stock");
//     }
    
//     Ok(())
// }

use reqwest;

use std::process::Command;

use oauth2::basic::BasicClient;

use oauth2::reqwest::async_http_client;
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, Scope, TokenResponse, TokenUrl};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;

use once_cell::sync::Lazy;
use config::Config;
use sled::Db;

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

const shop_name: &str = "mobriver";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let key = ["oauth_token_ebay_", shop_name].concat();
    let token: Tokens = if let Ok(Some(x)) = DB.get(&key) {
        serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
    } else {
        println!("Getting a eBay user permission for {}", shop_name);
        // auth().await;
        panic!()
        // auth(shop_name).unwrap()
    };
    // let access_token = refresh(token.refresh_token.clone()).unwrap();
    let access_token = &token.access_token;

    let url = "https://api.ebay.com/developer/analytics/v1_beta/rate_limit/?api_name=Analytics";
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Authorization", ["Bearer ", access_token].concat())
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;

        println!("{}", resp);

    if false {
        auth().await
    }

    DB.flush()?;
    Ok(())
}
async fn auth() {
    
    let auth_url = AuthUrl::new("https://auth.ebay.com/oauth2/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://api.ebay.com/identity/v1/oauth2/token".to_string())
        .expect("Invalid token endpoint URL");        

    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(
        ClientId::new((*EBAY_CLIENT_ID).clone()),
        Some(ClientSecret::new((*EBAY_CLIENT_SECRET).clone())),
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
            }
            break;
        }
    }
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
    // pub fn refresh() -> Result<Self, reqwest::Error> {
    //     let url = Url::parse(&format!("{}/oauth2/token", API_URL)).unwrap();
    //     let client = Client::builder()
    //         .timeout(Duration::from_secs(*TIMEOUT))
    //         .build()
    //         .unwrap();

    //     let refresh_token = "get_refresh_token()";
    //     let mut params = HashMap::new();
    //     params.insert("grant_type", "refresh_token");
    //     params.insert("client_id", &*EBAY_CLIENT_ID);
    //     params.insert("client_secret", &*EBAY_CLIENT_SECRET);
    //     params.insert("refresh_token", &refresh_token);
    //     let res = client.post(url).form(&params).send()?;

    //     println!("Exchanging the token... {}", res.status());
    //     let body: Self = res.json()?;
    //     Ok(body)
    // }
}
