use crate::*;

use serde::Deserialize;
use quick_xml::de::{from_str, DeError};
use rand::seq::SliceRandom;

pub async fn leave() -> Result<(), Box<dyn std::error::Error>> {

    let shops_for_feedback = CONF.get::<Vec<String>>("shops_for_feedback").unwrap();
    let comments = ["❤️Fast payment. Perfect! THANKS!❤️", "❤️Fast payment. Excellent buyer! THANKS!❤️"];

    for shop_name in shops_for_feedback {
        println!("{}", shop_name);
        let api_endpoint = "/ws/api.dll";

        let mut web = Web::new(&shop_name).await?;

        let limit = 10;
        let call_name = "GetItemsAwaitingFeedback";

        let body = format!(r#"
        <?xml version="1.0" encoding="utf-8"?>
        <{}Request xmlns="urn:ebay:apis:eBLBaseComponents">
          <Pagination>
            <EntriesPerPage>{}</EntriesPerPage>
            <PageNumber>{}</PageNumber>
          </Pagination>
        <Sort>FeedbackReceivedDescending</Sort>
        </GetItemsAwaitingFeedbackRequest>
        "#, call_name, limit, 1);

        let reply = web.post(api_endpoint, call_name, body).await?;

        let xml: GetItemsAwaitingFeedbackResponse = from_str(&reply)?;

        let all_feedback: Vec<Transaction> = xml.items_awaiting_feedback.transaction_array.transaction
            .into_iter()
            .filter(|feedback| feedback.feedback_received.is_some() && feedback.buyer.is_some())
            .collect();

        if all_feedback.is_empty() {
            println!("There is no feedback to reply to");
            continue; 
        }

        let positive: Vec<Transaction> = all_feedback
            .into_iter()
            .filter(|feedback| 
                if let Some(x) = &feedback.feedback_received {
                    x.comment_type == "Positive"
                } else {
                    false
                }
            )
            .collect();

        if positive.is_empty() {
            println!("There is no positive feedback to reply to");
            continue;
        }

        let call_name = "LeaveFeedback";

        for feedback in positive {
            let user_id = feedback.buyer.unwrap_or_default().user_id;
            let body = format!(r#"
            <?xml version="1.0" encoding="utf-8"?>
            <{}Request xmlns="urn:ebay:apis:eBLBaseComponents">
              <ItemID>{}</ItemID>
              <TransactionID>{}</TransactionID>
              <CommentText>{}</CommentText>
              <TargetUser>{}</TargetUser>
              <CommentType>Positive</CommentType>
            </LeaveFeedbackRequest>
            "#,
                call_name,
                feedback.item.item_id,
                feedback.transaction_id,
                comments.choose(&mut rand::thread_rng()).unwrap_or_else(|| &"Thanks!"),
                user_id,
            );
            let reply = web.post(api_endpoint, call_name, body).await?;
            if reply.contains("Success") {
                println!("{}... OK", user_id);
            } else {
                println!("{} - Error! {}", user_id, reply);
            }
        }

    }

    Ok(())
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

#[derive(Default, Debug, Deserialize, PartialEq)]
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
