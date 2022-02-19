use oauth2::RedirectUrl;

use crate::*;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FagentApi {
    pub tokens: Tokens,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub scope: Scope,
    pub limit: String,
}

pub const FAGENT_API_URL: &str = "https://api.freeagent.com/v2/";

impl FagentApi {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let key = "oauth_token_fagent";
        let tokens = if let Ok(Some(x)) = DB.get(&key) {
            serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
        } else {
            println!("No tokens found in the DB for Free Agent");
            Tokens::new(String::new(), 0, String::new())
        };

        Ok(FagentApi {
            tokens,
            auth_url: AuthUrl::new("https://api.freeagent.com/v2/approve_app".to_string())?,
            token_url: TokenUrl::new("https://api.freeagent.com/v2/token_endpoint".to_string())?,
            client_id: ClientId::new(CONF.get::<String>("free_agent.client_id").unwrap()),
            client_secret: ClientSecret::new(
                CONF.get::<String>("free_agent.client_secret").unwrap(),
            ),
            scope: Scope::new("fagent scope".to_string()),
            limit: CONF.get::<String>("free_agent.limit").unwrap().to_string(),
        })
    }

    pub async fn _get(&mut self, api_endpoint: &str) -> Result<String, Box<dyn std::error::Error>> {
        // переделать в метод структуры токенс
        let mut reply = String::new();
        for i in 1..=3 {
            // перенести эту проверку внутрь get
            let mut params: HashMap<&str, String> = HashMap::new();
            params.insert("limit", self.limit.clone());
            let url = [FAGENT_API_URL, api_endpoint].concat();
            let client = reqwest::Client::new();
            let res = client
               .get(url)
               .query(&params)
               .header("Authorization", ["Bearer ", &self.tokens.access_token].concat())
               .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0")
               .header("Content-Type", "application/json")
               .send()
               .await?;

            eprintln!("{}", res.status());

            reply = res.text().await?;
            if !reply.contains("error") {
                break;
            } else {
                let bad_token = "Access token not recognised";
                if reply.contains(bad_token) {
                    println!("{bad_token}");
                    match i {
                        1 => self.refresh_access_token(true).await?,
                        2 => self.auth().await?,
                        _ => println!("Error during token exchagne cycle"),
                    }
                } else {
                    println!("ebay_api.post has failed: {reply}");
                }
            }
        }
        Ok(reply)
    }

    pub async fn post(
        &mut self,
        api_endpoint: &str,
        body: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut reply = String::new();
        for i in 1..=3 {
            let url = [FAGENT_API_URL, api_endpoint].concat();
            let client = reqwest::Client::new();
            let res  = client
           .post(url)
           .body(body.clone())
           .header("Authorization", ["Bearer ", &self.tokens.access_token].concat())
           .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:79.0) Gecko/20100101 Firefox/79.0")
           .header("Content-Type", "application/json")
           .send()
           .await?;

            eprintln!("{}", res.status());

            reply = res.text().await?;
            if !reply.contains("rrors") {
                break;
            } else {
                let bad_token = "Access token not recognised";
                if reply.contains(bad_token) {
                    println!("{bad_token}");
                    match i {
                        1 => self.refresh_access_token(true).await?,
                        2 => self.auth().await?,
                        _ => println!("Error during token exchagne cycle"),
                    }
                } else {
                    println!("ebay_api.post has failed: {reply}");
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
        )
        .set_redirect_uri(
            RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
        );

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state) = client
            .authorize_url(|| CsrfToken::new("alsirua3w8awoirhj".to_string()))
            // .add_scope(self.scope.clone())
            .url();

        // let url = format!("start {}", authorize_url.to_string()).replace("&", "^&");

        // let output = Command::new("cmd.exe").args(["/C", &url]).output();

        // if let Err(e) = output {
        // println!("We couldn't start your browser: {:?}\n\n
        // Please start it manually and open this URL:\n{}\n", e, authorize_url.to_string());
        // }

        println!("Please open this URL:\n{}\n", authorize_url.to_string());

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
                    .request_async(async_http_client)
                    .await;

                println!(
                    "FreeAgent has returned the following token:\n{:?}\n",
                    token_res
                );

                if let Ok(token) = token_res {
                    // обрабатывать если вернул ошибку!!!
                    let tokens = Tokens::new(
                        token.access_token().secret().clone(),
                        token.expires_in().unwrap().as_secs(),
                        token.refresh_token().unwrap().secret().clone(),
                    );
                    dbg!(&tokens);

                    let key = "oauth_token_fagent".to_string();
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

        let res = client
            .exchange_refresh_token(&RefreshToken::new(self.tokens.refresh_token.clone()))
            //   .add_scope(self.scope.clone())
            .request_async(async_http_client)
            .await;

        // dbg!(&res);

        // if let Ok(token) = res {
        // обрабатывать если вернул ошибку!!!
        let token = match res {
            Ok(x) => x,
            Err(e) => {
                println!("Token refresh has failed with following error: {}", e);
                return Ok(());
            }
        };

        let tokens = Tokens::new(
            token.access_token().secret().clone(),
            token.expires_in().unwrap().as_secs(),
            self.tokens.refresh_token.to_string(),
        );

        let key = "oauth_token_fagent".to_string();
        DB.insert(&key, serde_json::to_string(&tokens).unwrap().as_bytes())
            .unwrap();

        self.tokens.access_token = token.access_token().secret().clone();
        if print {
            println!("OK")
        };
        Ok(())
    }
}
