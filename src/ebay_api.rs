use crate::*;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

// static EBAY_URL_SCHEME: Lazy<String> = Lazy::new(|| CONF.get::<String>("ebay.ru_name").unwrap());

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

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EbayApi {
    pub shop_name: String,
    pub tokens: Tokens,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub scope: Scope,
    pub url_sceme: String,
    pub limit: String,
}

impl EbayApi {
    pub async fn new(shop_name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let key = ["oauth_token_ebay_", shop_name].concat();
        let tokens = if let Ok(Some(x)) = DB.get(&key) {
            serde_json::from_str(std::str::from_utf8(&x)?)?
        } else {
            println!("No tokens found in the DB for {}", shop_name);
            Tokens::new(String::new(), 0, String::new())
        };

        // let tokens = Tokens::new(String::new(), 0, String::new());

        Ok(EbayApi {
            shop_name: shop_name.to_string(),
            tokens,
            auth_url: AuthUrl::new("https://auth.ebay.com/oauth2/authorize".to_string())?,
            token_url: TokenUrl::new("https://api.ebay.com/identity/v1/oauth2/token".to_string())?,
            client_id: ClientId::new(CONF.get::<String>("ebay.client_id")?),
            client_secret: ClientSecret::new(CONF.get::<String>("ebay.client_secret")?),
            scope: Scope::new("https://api.ebay.com/oauth/api_scope/sell.fulfillment https://api.ebay.com/oauth/api_scope".to_string()),
            url_sceme: CONF.get::<String>("ebay.ru_name")?,
            limit: CONF.get::<String>("ebay.limit")?.to_string(),
        })
    }

    pub async fn get(&mut self, api_endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
        // переделать в метод структуры токенс
        let mut reply = String::new();
        for i in 1..=3 {
            // перенести эту проверку внутрь get
            let mut params: HashMap<&str, String> = HashMap::new();
            params.insert("limit", self.limit.clone());
            let url = [EBAY_API_URL, api_endpoint].concat();
            let client = reqwest::Client::new();
            reply = client
                .get(url)
                .query(&params)
                .header(
                    "Authorization",
                    ["Bearer ", &self.tokens.access_token].concat(),
                )
                .header("Content-Type", "application/json")
                .send()
                .await?
                .text()
                .await?;
            if reply.contains("errorId") {
                let x = "Invalid access token";
                if reply.contains(x) {
                    println!("{x}");
                    match i {
                        1 => self.refresh_access_token(true).await?,
                        2 => self.auth().await?,
                        _ => return Err(LocalError::EbayTokenError)?,
                    }
                } else if reply.contains("System error") {
                    return Err(LocalError::EbaySystemError)?;
                } else {
                    return Err(LocalError::EbayUnknownError(reply))?;
                }
            }
        }
        Ok(reply)
    }

    pub async fn post(
        // переделать чтобы возвращал ошибки
        &mut self,
        api_endpoint: &str,
        call_name: &str,
        body: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut reply = String::new();
        for i in 1..=3 {
            let url = [EBAY_API_URL, api_endpoint].concat();
            let client = reqwest::Client::new();
            reply = client
                .post(url)
                .body(body.clone())
                .header("Content-Type", "text/xml")
                .header("X-EBAY-API-SITEID", "3")
                .header("X-EBAY-API-COMPATIBILITY-LEVEL", "1225")
                .header("X-EBAY-API-IAF-TOKEN", &self.tokens.access_token)
                .header("X-EBAY-API-CALL-NAME", call_name)
                .send()
                .await?
                .text()
                .await?;
            if reply.contains("rrors") {
                let a = "Invalid access token";

                if reply.contains(a) || reply.contains("IAF token supplied is expired") {
                    println!("{} - {}", self.shop_name, a);
                    match i {
                        1 => self.refresh_access_token(true).await?,
                        2 => self.auth().await?,
                        _ => return Err(LocalError::EbayTokenError)?,
                    }
                } else if reply.contains("or feedback already left") {
                    return Err(LocalError::EbayFeedbackAlreadyLeft)?;
                } else {
                    return Err(LocalError::EbayFeedbackUnknownError(reply))?;
                }
            }
        }
        Ok(reply)
    }

    pub async fn auth(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Set up the config for the Github OAuth2 process.
        let client = BasicClient::new(
            self.client_id.clone(),
            Some(self.client_secret.clone()),
            self.auth_url.clone(),
            Some(self.token_url.clone()),
        );

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client
            .authorize_url(|| CsrfToken::new("http://localhost:8080".to_string()))
            .add_scope(self.scope.clone())
            .add_extra_param("redirect_uri", &self.url_sceme)
            .url();

        // let url = format!("start {}", authorize_url.to_string()).replace("&", "^&");

        // let output = Command::new("cmd.exe").args(["/C", &url]).output();

        // if let Err(e) = output {
        // println!("We couldn't start your browser: {:?}\n\n
        // Please start it manually and open this URL:\n{}\n", e, authorize_url.to_string());
        // }

        println!("Please open this URL:\n{}\n", authorize_url.to_string());

        // A very naive implementation of the redirect server.
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        loop {
            if let Ok((mut stream, _)) = listener.accept().await {
                let code;
                let state;
                {
                    let mut reader = BufReader::new(&mut stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).await?;

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

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
                stream.write_all(response.as_bytes()).await?;

                println!("Request returned the following code:\n{}\n", code.secret());
                println!(
                    "Request returned the following state:\n{} (expected `{}`)\n",
                    state.secret(),
                    csrf_state.secret()
                );

                // Exchange the code with a token.
                let token_res = client
                    .exchange_code(code)
                    .add_extra_param("redirect_uri", &self.url_sceme)
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

                    let key = ["oauth_token_ebay_", &self.shop_name].concat();
                    DB.insert(&key, serde_json::to_string(&tokens).unwrap().as_bytes())
                        .unwrap();
                    self.tokens.access_token = token.access_token().secret().clone();
                }
                break;
            }
        }
        Ok(())
    }

    pub async fn refresh_access_token(
        &mut self,
        print: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if print {
            print!("Refreshing access token... ")
        };
        let client = BasicClient::new(
            self.client_id.clone(),
            Some(self.client_secret.clone()),
            self.auth_url.clone(),
            Some(self.token_url.clone()),
        );

        let token = client
            .exchange_refresh_token(&RefreshToken::new(self.tokens.refresh_token.clone()))
            .add_scope(self.scope.clone())
            .request_async(async_http_client)
            .await?;

        // dbg!(&res);

        let tokens = Tokens::new(
            token.access_token().secret().clone(),
            token.expires_in().unwrap().as_secs(),
            self.tokens.refresh_token.to_string(),
        );

        let key = ["oauth_token_ebay_", &self.shop_name].concat();
        DB.insert(&key, serde_json::to_string(&tokens)?.as_bytes())?;

        self.tokens.access_token = token.access_token().secret().clone();
        if print {
            println!("OK")
        };
        Ok(())
    }
}

pub async fn ws() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/messages", post(handle_ebay_message));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("listening on {}", addr);
    tokio::spawn(axum::Server::bind(&addr).serve(app.into_make_service()));

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
            <ApplicationEnable>Disable</ApplicationEnable>
            <ApplicationURL>https://ws.mobriver.co.uk/messages</ApplicationURL>
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
            <NotificationEnable>
              <EventType>ItemAddedToWatchList</EventType>
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

    println!("The second reply: {reply}");

    // tokio::time::sleep(Duration::from_secs(100)).await;

    Ok(())
}
