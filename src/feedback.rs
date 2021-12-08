use crate::*;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};

pub async fn leave() -> Result<(), Box<dyn std::error::Error>> {

    let shops_for_feedback = CONF.get::<Vec<String>>("shops_for_feedback").unwrap();

    for shop_name in shops_for_feedback {
        println!("{}", shop_name);
        let api_endpoint = "/ws/api.dll";

        let mut web = Web::new(&shop_name).await?;

        let reply = web.post(api_endpoint).await?;

        let xml: GetItemsAwaitingFeedbackResponse = from_str(&reply)?;

        for feedback in xml.items_awaiting_feedback.transaction_array.transaction {
            if feedback.feedback_received.is_some() && feedback.buyer.is_some() {
                println!("{:#?}", feedback);
            }
        }

        

    }

    Ok(())
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Welcome8 {
    #[serde(rename = "GetItemsAwaitingFeedbackResponse")]
    get_items_awaiting_feedback_response: GetItemsAwaitingFeedbackResponse,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct GetItemsAwaitingFeedbackResponse {
    #[serde(rename = "Timestamp")]
    timestamp: String,

    #[serde(rename = "Ack")]
    ack: String,

    #[serde(rename = "Version")]
    version: String,

    #[serde(rename = "Build")]
    build: String,

    #[serde(rename = "ItemsAwaitingFeedback")]
    items_awaiting_feedback: ItemsAwaitingFeedback,

    // #[serde(rename = "_xmlns")]
    // xmlns: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemsAwaitingFeedback {
    #[serde(rename = "TransactionArray")]
    transaction_array: TransactionArray,

    #[serde(rename = "PaginationResult")]
    pagination_result: PaginationResult,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PaginationResult {
    #[serde(rename = "TotalNumberOfPages")]
    total_number_of_pages: String,

    #[serde(rename = "TotalNumberOfEntries")]
    total_number_of_entries: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TransactionArray {
    #[serde(rename = "Transaction")]
    transaction: Vec<Transaction>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Transaction {
    #[serde(rename = "Item")]
    item: Item,

    #[serde(rename = "TransactionID")]
    transaction_id: String,

    #[serde(rename = "FeedbackReceived")]
    feedback_received: Option<FeedbackReceived>,

    #[serde(rename = "OrderLineItemID")]
    order_line_item_id: String,

    #[serde(rename = "Buyer")]
    buyer: Option<Buyer>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Buyer {
    #[serde(rename = "UserID")]
    user_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct FeedbackReceived {
    #[serde(rename = "CommentType")]
    comment_type: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Item {
    #[serde(rename = "ItemID")]
    item_id: String,

    #[serde(rename = "ListingDetails")]
    listing_details: ListingDetails,

    #[serde(rename = "Seller")]
    seller: Option<Buyer>,

    #[serde(rename = "Title")]
    title: String,

    #[serde(rename = "ConditionID")]
    condition_id: String,

    #[serde(rename = "ConditionDisplayName")]
    condition_display_name: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ListingDetails {
    #[serde(rename = "EndTime")]
    end_time: String,
}
