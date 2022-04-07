use crate::*;

use axum::{
    routing::{get, post},
    Router,
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

    pub async fn post2(
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
                // .header("X-EBAY-API-IAF-TOKEN", &self.tokens.access_token)
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

pub async fn set_notifications() -> Result<(), Box<dyn std::error::Error>> {
    let shop_name = "mobriver";

    let api_endpoint = "/ws/api.dll";
    let db_key = format!("ebay_ana_{shop_name}");
    let mut ebay_api = EbayApi::new(&shop_name).await?;

    // first call
    let ana_token = CONF.get::<String>(&db_key)?;
    let call_name = "SetNotificationPreferences";
    let body = format!(
        r#"
        <?xml version="1.0" encoding="utf-8"?>
        <SetNotificationPreferencesRequest xmlns="urn:ebay:apis:eBLBaseComponents">
          <RequesterCredentials>
            <eBayAuthToken>{ana_token}</eBayAuthToken>
          </RequesterCredentials>
          <ApplicationDeliveryPreferences>
            <AlertEmail>mailto://andy4usa@gmail.com</AlertEmail>
            <AlertEnable>Enable</AlertEnable>
            <ApplicationEnable>Enable</ApplicationEnable>
            <ApplicationURL>https://ws.mobriver.co.uk/messages</ApplicationURL>
            <DeviceType>Platform</DeviceType>
          </ApplicationDeliveryPreferences>
          <UserDeliveryPreferenceArray>
            <NotificationEnable>
              <EventType>FeedbackReceived</EventType>
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
    let reply = ebay_api.post2(api_endpoint, call_name, body).await?;

    println!("\nThe first reply: {reply}\n");

    // second call

    let call_name = "GetNotificationPreferences";
    let body = format!(
        r#"
            <?xml version="1.0" encoding="utf-8"?>
            <GetNotificationPreferencesRequest xmlns="urn:ebay:apis:eBLBaseComponents">
                <PreferenceLevel>User</PreferenceLevel>
            </GetNotificationPreferencesRequest>
        "# // r#"
           // <?xml version="1.0" encoding="utf-8"?>
           // <GetNotificationsUsageRequest xmlns="urn:ebay:apis:eBLBaseComponents">
           //     <ErrorLanguage>en_US</ErrorLanguage>
           // </GetNotificationsUsageRequest>
           // "#
    );
    let reply = ebay_api.post(api_endpoint, call_name, body).await?;

    println!("The second reply: {reply}");

    // panic!("ouch!");
    // tokio::time::sleep(Duration::from_secs(100)).await;

    Ok(())
}

pub async fn ws() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(handle_get))
        .route("/messages", post(handle_ebay_message));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    tokio::spawn(axum::Server::bind(&addr).serve(app.into_make_service()));

    // tokio::time::sleep(Duration::from_secs(1000)).await; // sleep ----------------------------------------------------------------------------------------

    Ok(())
}

async fn handle_get() -> &'static str {
    "200 OK"
}

async fn handle_ebay_message(payload: String) -> &'static str {
    let soap_start_marker = "<soapenv:Body>";
    let soap_end_marker = "</soapenv:Body>";
    let header = if let Some(x) = payload.find(soap_start_marker) {
        x
    } else {
        println!("This is not a SOAP message\n\nPayload: {payload}\n");
        return "This is not a SOAP message";
    };

    let footer = if let Some(x) = payload.find(soap_end_marker) {
        x
    } else {
        println!("This is not a SOAP message\n\nPayload: {payload}\n");
        return "This is not a SOAP message";
    };

    let xml_str = &payload[(header + soap_start_marker.len()..footer)];

    let xml: SOAPMessageBody = match from_str(xml_str) {
        Ok(xml) => xml,
        Err(e) => {
            println!("SOAPMessage deserealisation error: {e}\n\nXML body: {xml_str}\n");
            return "error 5532";
        }
    };

    match xml {
        SOAPMessageBody::NewOrder(x) => {
            let shop_name = x.recipient_user_id;
            let order_id = x.transaction_array.transaction.containing_order.order_id;
            let item = x.item.title;
            let total: f64 = x
                .transaction_array
                .transaction
                .amount_paid
                .value
                .clone()
                .parse()
                .unwrap_or_default();
            let msg = format!("£{total} - {shop_name} - {item}");

            if total > CONF.get::<f64>("ebay.sale_to_notify").unwrap_or_default() {
                let mut orders: HashSet<String> = if let Ok(Some(x)) = DB.get("orders") {
                    serde_json::from_str(std::str::from_utf8(&x).unwrap()).unwrap()
                } else {
                    HashSet::new()
                };

                if !orders.contains(&order_id) {
                    println!("{msg}");
                    orders.insert(order_id);
                    DB.insert("orders", serde_json::to_string(&orders).unwrap().as_bytes())
                        .unwrap();

                    let url = format!("{TELEGRAM_URL}{msg}");
                    if let Err(e) = reqwest::get(url).await.unwrap().text().await {
                        println!(
                            "{shop_name} - Err68: sending a message to telegram has failed - {e}"
                        );
                    }
                } else {
                    println!("Order {order_id} for {item} on £{total} was already in the database when the message arrived");
                }
            }
        }
        SOAPMessageBody::NewFeedback(x) => {
            if x.feedback_detail_array.feedback_detail.role == "Seller"
                && x.feedback_detail_array.feedback_detail.comment_type == "Positive"
            {}
            println!(
                "New Feedback: {} - {} - {} - {}",
                x.recipient_user_id,
                x.feedback_detail_array.feedback_detail.role,
                x.feedback_detail_array.feedback_detail.commenting_user,
                x.feedback_detail_array.feedback_detail.comment_text
            );
        }
    };

    "OK"
}

extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub enum SOAPMessageBody {
    #[serde(rename = "GetItemTransactionsResponse")]
    NewOrder(GetItemTransactionsResponse),
    #[serde(rename = "GetFeedbackResponse")]
    NewFeedback(GetFeedbackResponse),
}

#[derive(Serialize, Deserialize)]
pub struct GetFeedbackResponse {
    #[serde(rename = "Timestamp")]
    timestamp: String,

    #[serde(rename = "Ack")]
    ack: String,

    #[serde(rename = "CorrelationID")]
    correlation_id: String,

    #[serde(rename = "Version")]
    version: String,

    #[serde(rename = "Build")]
    build: String,

    #[serde(rename = "NotificationEventName")]
    notification_event_name: String,

    #[serde(rename = "RecipientUserID")]
    recipient_user_id: String,

    #[serde(rename = "EIASToken")]
    eias_token: String,

    #[serde(rename = "FeedbackDetailArray")]
    feedback_detail_array: FeedbackDetailArray,

    #[serde(rename = "FeedbackDetailItemTotal")]
    feedback_detail_item_total: String,

    #[serde(rename = "FeedbackScore")]
    feedback_score: String,

    #[serde(rename = "PaginationResult")]
    pagination_result: PaginationResult,

    #[serde(rename = "EntriesPerPage")]
    entries_per_page: String,

    #[serde(rename = "PageNumber")]
    page_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct FeedbackDetailArray {
    #[serde(rename = "FeedbackDetail")]
    feedback_detail: FeedbackDetail,
}

#[derive(Serialize, Deserialize)]
pub struct FeedbackDetail {
    #[serde(rename = "CommentingUser")]
    commenting_user: String,

    #[serde(rename = "FeedbackRatingStar")]
    feedback_rating_star: String,

    #[serde(rename = "CommentingUserScore")]
    commenting_user_score: String,

    #[serde(rename = "CommentText")]
    comment_text: String,

    #[serde(rename = "CommentTime")]
    comment_time: String,

    #[serde(rename = "CommentType")]
    comment_type: String,

    #[serde(rename = "ItemID")]
    item_id: String,

    #[serde(rename = "Role")]
    role: String,

    #[serde(rename = "FeedbackID")]
    feedback_id: String,

    #[serde(rename = "TransactionID")]
    transaction_id: String,

    #[serde(rename = "OrderLineItemID")]
    order_line_item_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetItemTransactionsResponse {
    #[serde(rename = "Timestamp")]
    timestamp: String,

    #[serde(rename = "Ack")]
    ack: String,

    #[serde(rename = "CorrelationID")]
    correlation_id: String,

    #[serde(rename = "Version")]
    version: String,

    #[serde(rename = "Build")]
    build: String,

    #[serde(rename = "NotificationEventName")]
    notification_event_name: String,

    #[serde(rename = "RecipientUserID")]
    recipient_user_id: String,

    #[serde(rename = "EIASToken")]
    eias_token: String,

    #[serde(rename = "PaginationResult")]
    pagination_result: PaginationResult,

    #[serde(rename = "HasMoreTransactions")]
    has_more_transactions: String,

    #[serde(rename = "TransactionsPerPage")]
    transactions_per_page: String,

    #[serde(rename = "PageNumber")]
    page_number: String,

    #[serde(rename = "ReturnedTransactionCountActual")]
    returned_transaction_count_actual: String,

    #[serde(rename = "Item")]
    item: Item,

    #[serde(rename = "TransactionArray")]
    transaction_array: TransactionArray,

    #[serde(rename = "PayPalPreferred")]
    pay_pal_preferred: String,

    #[serde(rename = "xmlns")]
    xmlns: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "AutoPay")]
    auto_pay: String,

    #[serde(rename = "BuyItNowPrice")]
    buy_it_now_price: BuyItNowPrice,

    #[serde(rename = "Currency")]
    currency: String,

    #[serde(rename = "ItemID")]
    item_id: String,

    #[serde(rename = "ListingDetails")]
    listing_details: ListingDetails,

    #[serde(rename = "ListingType")]
    listing_type: String,

    #[serde(rename = "PrimaryCategory")]
    primary_category: AryCategory,

    #[serde(rename = "PrivateListing")]
    private_listing: String,

    #[serde(rename = "Quantity")]
    quantity: String,

    #[serde(rename = "SecondaryCategory")]
    secondary_category: AryCategory,

    #[serde(rename = "Seller")]
    seller: Seller,

    #[serde(rename = "SellingStatus")]
    selling_status: SellingStatus,

    #[serde(rename = "Site")]
    site: String,

    #[serde(rename = "StartPrice")]
    start_price: BuyItNowPrice,

    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "GetItFast")]
    get_it_fast: String,

    #[serde(rename = "SKU")]
    sku: Option<String>,

    #[serde(rename = "IntegratedMerchantCreditCardEnabled")]
    integrated_merchant_credit_card_enabled: String,

    #[serde(rename = "ConditionID")]
    condition_id: String,

    #[serde(rename = "ConditionDisplayName")]
    condition_display_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyItNowPrice {
    #[serde(rename = "currencyID")]
    currency_id: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListingDetails {
    #[serde(rename = "StartTime")]
    start_time: String,

    #[serde(rename = "EndTime")]
    end_time: String,

    #[serde(rename = "ViewItemURL")]
    view_item_url: String,

    #[serde(rename = "ViewItemURLForNaturalSearch")]
    view_item_url_for_natural_search: String,
}

#[derive(Serialize, Deserialize)]
pub struct AryCategory {
    #[serde(rename = "CategoryID")]
    category_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Seller {
    #[serde(rename = "AboutMePage")]
    about_me_page: String,

    #[serde(rename = "EIASToken")]
    eias_token: String,

    #[serde(rename = "Email")]
    email: String,

    #[serde(rename = "FeedbackScore")]
    feedback_score: String,

    #[serde(rename = "PositiveFeedbackPercent")]
    positive_feedback_percent: String,

    #[serde(rename = "FeedbackPrivate")]
    feedback_private: String,

    #[serde(rename = "IDVerified")]
    id_verified: String,

    #[serde(rename = "eBayGoodStanding")]
    e_bay_good_standing: String,

    #[serde(rename = "NewUser")]
    new_user: String,

    #[serde(rename = "RegistrationDate")]
    registration_date: String,

    #[serde(rename = "Site")]
    site: String,

    #[serde(rename = "Status")]
    status: String,

    #[serde(rename = "UserID")]
    user_id: String,

    #[serde(rename = "UserIDChanged")]
    user_id_changed: String,

    #[serde(rename = "UserIDLastChanged")]
    user_id_last_changed: Option<String>,

    #[serde(rename = "VATStatus")]
    vat_status: String,

    #[serde(rename = "SellerInfo")]
    seller_info: Option<SellerInfo>,

    #[serde(rename = "BuyerInfo")]
    buyer_info: Option<BuyerInfo>,

    #[serde(rename = "UserAnonymized")]
    user_anonymized: Option<String>,

    #[serde(rename = "StaticAlias")]
    static_alias: Option<String>,

    #[serde(rename = "UserFirstName")]
    user_first_name: Option<String>,

    #[serde(rename = "UserLastName")]
    user_last_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BuyerInfo {
    #[serde(rename = "ShippingAddress")]
    shipping_address: ShippingAddress,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingAddress {
    #[serde(rename = "Name")]
    name: Option<String>,

    #[serde(rename = "Street1")]
    street1: Option<String>,

    #[serde(rename = "Street2")]
    street2: Option<String>,

    #[serde(rename = "CityName")]
    city_name: Option<String>,

    #[serde(rename = "StateOrProvince")]
    state_or_province: Option<String>,

    #[serde(rename = "Country")]
    country: String,

    #[serde(rename = "CountryName")]
    country_name: String,

    #[serde(rename = "Phone")]
    phone: Option<String>,

    #[serde(rename = "PostalCode")]
    postal_code: String,

    #[serde(rename = "AddressID")]
    address_id: String,

    #[serde(rename = "AddressOwner")]
    address_owner: String,

    #[serde(rename = "AddressUsage")]
    address_usage: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellerInfo {
    #[serde(rename = "AllowPaymentEdit")]
    allow_payment_edit: String,

    #[serde(rename = "CheckoutEnabled")]
    checkout_enabled: String,

    #[serde(rename = "CIPBankAccountStored")]
    cip_bank_account_stored: String,

    #[serde(rename = "GoodStanding")]
    good_standing: String,

    #[serde(rename = "LiveAuctionAuthorized")]
    live_auction_authorized: String,

    #[serde(rename = "MerchandizingPref")]
    merchandizing_pref: String,

    #[serde(rename = "QualifiesForB2BVAT")]
    qualifies_for_b2_bvat: String,

    #[serde(rename = "StoreOwner")]
    store_owner: String,

    #[serde(rename = "StoreURL")]
    store_url: Option<String>,

    #[serde(rename = "SafePaymentExempt")]
    safe_payment_exempt: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellingStatus {
    #[serde(rename = "ConvertedCurrentPrice")]
    converted_current_price: BuyItNowPrice,

    #[serde(rename = "CurrentPrice")]
    current_price: BuyItNowPrice,

    #[serde(rename = "QuantitySold")]
    quantity_sold: String,

    #[serde(rename = "ListingStatus")]
    listing_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct PaginationResult {
    #[serde(rename = "TotalNumberOfPages")]
    total_number_of_pages: String,

    #[serde(rename = "TotalNumberOfEntries")]
    total_number_of_entries: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransactionArray {
    #[serde(rename = "Transaction")]
    transaction: Transaction,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    #[serde(rename = "AmountPaid")]
    amount_paid: BuyItNowPrice,

    #[serde(rename = "AdjustmentAmount")]
    adjustment_amount: BuyItNowPrice,

    #[serde(rename = "ConvertedAdjustmentAmount")]
    converted_adjustment_amount: BuyItNowPrice,

    #[serde(rename = "Buyer")]
    buyer: Seller,

    #[serde(rename = "ShippingDetails")]
    shipping_details: ShippingDetails,

    #[serde(rename = "ConvertedAmountPaid")]
    converted_amount_paid: BuyItNowPrice,

    #[serde(rename = "ConvertedTransactionPrice")]
    converted_transaction_price: BuyItNowPrice,

    #[serde(rename = "CreatedDate")]
    created_date: String,

    #[serde(rename = "DepositType")]
    deposit_type: String,

    #[serde(rename = "QuantityPurchased")]
    quantity_purchased: String,

    #[serde(rename = "Status")]
    status: Status,

    #[serde(rename = "TransactionID")]
    transaction_id: String,

    #[serde(rename = "TransactionPrice")]
    transaction_price: BuyItNowPrice,

    #[serde(rename = "BestOfferSale")]
    best_offer_sale: String,

    #[serde(rename = "eBayCollectAndRemitTax")]
    e_bay_collect_and_remit_tax: String,

    #[serde(rename = "ExternalTransaction")]
    external_transaction: ExternalTransaction,

    #[serde(rename = "ShippingServiceSelected")]
    shipping_service_selected: ShippingServiceSelected,

    #[serde(rename = "BuyerMessage")]
    buyer_message: String,

    #[serde(rename = "PaidTime")]
    paid_time: String,

    #[serde(rename = "ContainingOrder")]
    containing_order: ContainingOrder,

    #[serde(rename = "TransactionSiteID")]
    transaction_site_id: String,

    #[serde(rename = "Platform")]
    platform: String,

    #[serde(rename = "PayPalEmailAddress")]
    pay_pal_email_address: String,

    #[serde(rename = "BuyerGuaranteePrice")]
    buyer_guarantee_price: BuyItNowPrice,

    #[serde(rename = "ActualShippingCost")]
    actual_shipping_cost: BuyItNowPrice,

    #[serde(rename = "OrderLineItemID")]
    order_line_item_id: String,

    #[serde(rename = "IsMultiLegShipping")]
    is_multi_leg_shipping: String,

    #[serde(rename = "IntangibleItem")]
    intangible_item: String,

    #[serde(rename = "MonetaryDetails")]
    monetary_details: MonetaryDetails,

    #[serde(rename = "ExtendedOrderID")]
    extended_order_id: String,

    #[serde(rename = "eBayPlusTransaction")]
    e_bay_plus_transaction: String,

    #[serde(rename = "GuaranteedShipping")]
    guaranteed_shipping: String,

    #[serde(rename = "GuaranteedDelivery")]
    guaranteed_delivery: String,
}

#[derive(Serialize, Deserialize)]
pub struct ContainingOrder {
    #[serde(rename = "OrderID")]
    order_id: String,

    #[serde(rename = "OrderStatus")]
    order_status: String,

    #[serde(rename = "CancelStatus")]
    cancel_status: String,

    #[serde(rename = "ExtendedOrderID")]
    extended_order_id: String,

    #[serde(rename = "ContainseBayPlusTransaction")]
    containse_bay_plus_transaction: String,

    #[serde(rename = "OrderLineItemCount")]
    order_line_item_count: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExternalTransaction {
    #[serde(rename = "ExternalTransactionID")]
    external_transaction_id: String,

    #[serde(rename = "ExternalTransactionTime")]
    external_transaction_time: String,

    #[serde(rename = "FeeOrCreditAmount")]
    fee_or_credit_amount: BuyItNowPrice,

    #[serde(rename = "PaymentOrRefundAmount")]
    payment_or_refund_amount: BuyItNowPrice,

    #[serde(rename = "ExternalTransactionStatus")]
    external_transaction_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct MonetaryDetails {
    #[serde(rename = "Payments")]
    payments: Payments,
}

#[derive(Serialize, Deserialize)]
pub struct Payments {
    #[serde(rename = "Payment")]
    payment: Payment,
}

#[derive(Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "PaymentStatus")]
    payment_status: String,

    #[serde(rename = "Payer")]
    payer: Payee,

    #[serde(rename = "Payee")]
    payee: Payee,

    #[serde(rename = "PaymentTime")]
    payment_time: String,

    #[serde(rename = "PaymentAmount")]
    payment_amount: BuyItNowPrice,

    #[serde(rename = "ReferenceID")]
    reference_id: Payee,

    #[serde(rename = "FeeOrCreditAmount")]
    fee_or_credit_amount: BuyItNowPrice,
}

#[derive(Serialize, Deserialize)]
pub struct Payee {
    #[serde(rename = "type")]
    payee_type: Option<String>,
    #[serde(rename = "$value")]
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingDetails {
    #[serde(rename = "ChangePaymentInstructions")]
    change_payment_instructions: String,

    #[serde(rename = "PaymentEdited")]
    payment_edited: String,

    #[serde(rename = "SalesTax")]
    sales_tax: SalesTax,

    #[serde(rename = "ShippingServiceOptions")]
    shipping_service_options: ShippingServiceOptions,

    #[serde(rename = "ShippingType")]
    shipping_type: String,

    #[serde(rename = "SellingManagerSalesRecordNumber")]
    selling_manager_sales_record_number: String,

    #[serde(rename = "ThirdPartyCheckout")]
    third_party_checkout: String,

    #[serde(rename = "TaxTable")]
    tax_table: String,
}

#[derive(Serialize, Deserialize)]
pub struct SalesTax {
    #[serde(rename = "SalesTaxPercent")]
    sales_tax_percent: String,

    #[serde(rename = "ShippingIncludedInTax")]
    shipping_included_in_tax: String,

    #[serde(rename = "SalesTaxAmount")]
    sales_tax_amount: BuyItNowPrice,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingServiceOptions {
    #[serde(rename = "ShippingService")]
    shipping_service: String,

    #[serde(rename = "ShippingServiceCost")]
    shipping_service_cost: BuyItNowPrice,

    #[serde(rename = "ShippingServiceAdditionalCost")]
    shipping_service_additional_cost: BuyItNowPrice,

    #[serde(rename = "ShippingServicePriority")]
    shipping_service_priority: String,

    #[serde(rename = "ExpeditedService")]
    expedited_service: String,

    #[serde(rename = "ShippingTimeMin")]
    shipping_time_min: String,

    #[serde(rename = "ShippingTimeMax")]
    shipping_time_max: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingServiceSelected {
    #[serde(rename = "ShippingService")]
    shipping_service: String,

    #[serde(rename = "ShippingServiceCost")]
    shipping_service_cost: BuyItNowPrice,

    #[serde(rename = "ShippingPackageInfo")]
    shipping_package_info: ShippingPackageInfo,
}

#[derive(Serialize, Deserialize)]
pub struct ShippingPackageInfo {
    #[serde(rename = "EstimatedDeliveryTimeMin")]
    estimated_delivery_time_min: String,

    #[serde(rename = "EstimatedDeliveryTimeMax")]
    estimated_delivery_time_max: String,
}

#[derive(Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "eBayPaymentStatus")]
    e_bay_payment_status: String,

    #[serde(rename = "CheckoutStatus")]
    checkout_status: String,

    #[serde(rename = "LastTimeModified")]
    last_time_modified: String,

    #[serde(rename = "PaymentMethodUsed")]
    payment_method_used: String,

    #[serde(rename = "CompleteStatus")]
    complete_status: String,

    #[serde(rename = "BuyerSelectedShipping")]
    buyer_selected_shipping: String,

    #[serde(rename = "PaymentHoldStatus")]
    payment_hold_status: String,

    #[serde(rename = "IntegratedMerchantCreditCardEnabled")]
    integrated_merchant_credit_card_enabled: String,

    #[serde(rename = "InquiryStatus")]
    inquiry_status: String,

    #[serde(rename = "ReturnStatus")]
    return_status: String,

    #[serde(rename = "PaymentInstrument")]
    payment_instrument: String,
}
