use crate::*;

use quick_xml::de::from_str;
use rand::seq::SliceRandom;
use serde::Deserialize;
// use sled::transaction;
// use serde_json::Error;

pub async fn leave() -> Result<(), Box<dyn std::error::Error>> {
    let shops_for_feedback = CONF.get::<Vec<String>>("shops_for_feedback")?;
    let comments = [
        "❤️Fast payment. Perfect! THANKS!❤️",
        "❤️Fast payment. Excellent buyer! THANKS!❤️",
    ];

    for shop_name in shops_for_feedback {
        let api_endpoint = "/ws/api.dll";

        let mut ebay_api = EbayApi::new(&shop_name).await?;

        let limit = 10;
        let call_name = "GetItemsAwaitingFeedback";

        let body = format!(
            r#"
        <?xml version="1.0" encoding="utf-8"?>
        <{}Request xmlns="urn:ebay:apis:eBLBaseComponents">
          <Pagination>
            <EntriesPerPage>{}</EntriesPerPage>
            <PageNumber>{}</PageNumber>
          </Pagination>
        <Sort>FeedbackReceivedDescending</Sort>
        </GetItemsAwaitingFeedbackRequest>
        "#,
            call_name, limit, 1
        );

        let reply = match ebay_api.post(api_endpoint, call_name, body).await {
            Ok(x) => x,
            Err(e) => {
                println!("{shop_name} - GetItemsAwaitingFeedback failed: {e}");
                continue;
            }
        };

        let xml: GetItemsAwaitingFeedbackResponse = match from_str(&reply) {
            Ok(xml) => xml,
            Err(e) => {
                println!(
                    "{} - GetItemsAwaitingFeedback XML Deserealisation error: {}\n\nXML body: {}\n",
                    shop_name, e, reply
                );
                continue;
            }
        };

        let items_awaiting_feedback = if let Some(t) = xml.items_awaiting_feedback {
            t
        } else {
            println!("{shop_name} - No awaiting feedback found as items_awaiting_feedback is null");
            continue;
        };

        // let i = items_awaiting_feedback
        //     .pagination_result
        //     .total_number_of_entries;

        // println!(
        //     "items_awaiting_feedback.pagination_result.total_number_of_entries: {shop_name} - {i}"
        // );

        let transaction_array = if let Some(t) = items_awaiting_feedback.transaction_array {
            t
        } else {
            println!("{shop_name} - No awaiting feedback found as transaction_array is empty");
            continue;
        };

        let all_feedback: Vec<Transaction> = transaction_array
            .transaction
            .into_iter()
            .filter(|feedback| feedback.feedback_received.is_some() && feedback.buyer.is_some())
            .collect();

        if all_feedback.is_empty() {
            continue;
        }

        let positive: Vec<Transaction> = all_feedback
            .into_iter()
            .filter(|feedback| {
                if let Some(x) = &feedback.feedback_received {
                    x.comment_type == "Positive"
                } else {
                    false
                }
            })
            .collect();

        if positive.is_empty() {
            continue;
        }

        let call_name = "LeaveFeedback";

        for feedback in positive {
            let user_id = feedback.buyer.unwrap_or_default().user_id;
            let body = format!(
                r#"
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
                comments
                    .choose(&mut rand::thread_rng())
                    .unwrap_or_else(|| &"Thanks!"),
                user_id,
            );

            let reply = match ebay_api.post(api_endpoint, call_name, body).await {
                Ok(x) => x,
                Err(e) => {
                    println!("{shop_name} - LeaveFeedback failed: {e}");
                    continue;
                }
            };

            if reply.contains("Success") {
                println!("{} - {}... OK", shop_name, user_id);
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
    items_awaiting_feedback: Option<ItemsAwaitingFeedback>,
    // #[serde(rename = "_xmlns")]
    // xmlns: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemsAwaitingFeedback {
    #[serde(rename = "TransactionArray")]
    transaction_array: Option<TransactionArray>,

    #[serde(rename = "PaginationResult")]
    pagination_result: PaginationResult,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct PaginationResult {
    #[serde(rename = "TotalNumberOfPages")]
    total_number_of_pages: String,

    #[serde(rename = "TotalNumberOfEntries")]
    total_number_of_entries: i32,
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
    condition_id: Option<String>,

    #[serde(rename = "ConditionDisplayName")]
    condition_display_name: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ListingDetails {
    #[serde(rename = "EndTime")]
    end_time: String,
}
