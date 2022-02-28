use crate::*;

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Order {
    pub channel: String,
    pub shop: String,
    pub buyer_id: String,
    pub buyer_name: String,
    pub note: String,
}

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OurEbayOrder {
    pub channel: String,
    pub shop: String,
    pub buyer_id: String,
    pub buyer_name: String,
    pub creation_date: DateTime<Utc>,
    pub items: String,
    pub total: f64,
    pub fulfillment: FulfillmentStartInstruction,
    pub note: Option<String>,
}

#[derive(Debug, Clone, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EtsyShop {
    pub name: String,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EtsyTransaction {
    pub count: i64,
    pub results: Vec<Transaction>,
    pub params: Params,
    #[serde(rename = "type")]
    pub type_field: String,
    pub pagination: Pagination,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EtsyReceipt {
    pub count: i64,
    pub results: Vec<Receipt>,
    pub params: Params,
    #[serde(rename = "type")]
    pub type_field: String,
    pub pagination: Pagination,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FallibleReceipts {
    pub count: i64,
    pub results: Vec<FallibleReceipt>,
    pub params: Params,
    #[serde(rename = "type")]
    pub type_field: String,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(untagged)]
pub enum FallibleReceipt {
    Ok(Receipt),
    Err(EtsyError),
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EtsyError {
    pub error_messages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Transaction {
    pub transaction_id: i64,
    pub title: String,
    pub description: String,
    pub seller_user_id: i64,
    pub buyer_user_id: i64,
    pub creation_tsz: i64,
    pub paid_tsz: Option<i64>,
    pub shipped_tsz: Option<i64>,
    pub price: String,
    pub currency_code: String,
    pub quantity: i64,
    pub tags: Vec<String>,
    pub materials: Vec<::serde_json::Value>,
    pub image_listing_id: i64,
    pub receipt_id: i64,
    pub shipping_cost: String,
    pub is_digital: bool,
    pub file_data: String,
    pub listing_id: i64,
    pub is_quick_sale: bool,
    pub seller_feedback_id: ::serde_json::Value,
    pub buyer_feedback_id: ::serde_json::Value,
    pub transaction_type: String,
    pub url: String,
    pub variations: Vec<Variation>,
    pub product_data: ProductData,
}
#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Receipt {
    pub receipt_id: i64,
    pub receipt_type: i64,
    pub order_id: i64,
    pub seller_user_id: i64,
    pub buyer_user_id: i64,
    pub creation_tsz: i64,
    pub can_refund: bool,
    pub last_modified_tsz: i64,
    pub name: String,
    pub first_line: String,
    pub second_line: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub zip: String,
    pub formatted_address: String,
    pub country_id: i64,
    pub payment_method: String,
    pub payment_email: String,
    pub message_from_seller: Option<String>,
    pub message_from_buyer: Option<String>,
    pub was_paid: bool,
    pub total_tax_cost: String,
    pub total_vat_cost: String,
    pub total_price: String,
    pub total_shipping_cost: String,
    pub currency_code: String,
    pub message_from_payment: Option<String>,
    pub was_shipped: bool,
    pub buyer_email: String,
    pub seller_email: String,
    pub is_gift: bool,
    pub needs_gift_wrap: bool,
    pub gift_message: String,
    pub discount_amt: String,
    pub subtotal: String,
    pub grandtotal: String,
    pub adjusted_grandtotal: String,
    pub buyer_adjusted_grandtotal: String,
    pub shipments: Vec<Shipment>,
    pub shipped_date: Option<i64>,
    pub is_overdue: bool,
    pub days_from_due_date: i64,
    pub shipping_details: ShippingDetails,
    pub transparent_price_message: String,
    pub show_channel_badge: bool,
    pub channel_badge_suffix_string: String,
    pub is_dead: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Shipment {
    pub receipt_shipping_id: ::serde_json::Value,
    pub mailing_date: i64,
    pub carrier_name: ::serde_json::Value,
    pub tracking_code: ::serde_json::Value,
    pub major_tracking_state: String,
    pub current_step: ::serde_json::Value,
    pub current_step_date: ::serde_json::Value,
    pub mail_class: ::serde_json::Value,
    pub buyer_note: String,
    pub notification_date: ::serde_json::Value,
    pub is_etsy_only_tracking: bool,
    pub tracking_url: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShippingDetails {
    pub can_mark_as_shipped: bool,
    pub was_shipped: bool,
    pub is_future_shipment: bool,
    pub shipment_date: Option<i64>,
    pub has_free_shipping_discount: bool,
    pub not_shipped_state_display: String,
    pub shipping_method: String,
    pub is_estimated_delivery_date_relevant: bool,
    pub estimated_delivery_date: Option<String>,
    pub delivery_date: Option<String>,
    pub is_delivered: Option<bool>,
    pub is_german_user: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Params {
    pub shop_id: Option<String>,
    pub min_created: Option<::serde_json::Value>,
    pub max_created: Option<::serde_json::Value>,
    pub min_last_modified: Option<::serde_json::Value>,
    pub max_last_modified: Option<::serde_json::Value>,
    pub limit: String,
    pub offset: i64,
    pub page: ::serde_json::Value,
    pub was_paid: Option<::serde_json::Value>,
    pub was_shipped: Option<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Pagination {
    pub effective_limit: i64,
    pub effective_offset: i64,
    pub next_offset: Option<i64>,
    pub effective_page: i64,
    pub next_page: Option<i64>,
}

// Transaction

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ProductData {
    pub product_id: i64,
    pub sku: String,
    pub property_values: Vec<::serde_json::Value>,
    pub offerings: Vec<Offering>,
    pub is_deleted: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Offering {
    pub offering_id: i64,
    pub price: Price,
    pub quantity: i64,
    pub is_enabled: i64,
    pub is_deleted: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Price {
    pub amount: i64,
    pub divisor: i64,
    pub currency_code: String,
    pub currency_formatted_short: String,
    pub currency_formatted_long: String,
    pub currency_formatted_raw: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Variation {
    pub property_id: i64,
    pub value_id: ::serde_json::Value,
    pub formatted_name: String,
    pub formatted_value: String,
}

// Royal Mail

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RMCommonData {
    #[serde(rename = "antiForgeryToken")]
    pub anti_forgery_token: String,
    #[serde(rename = "accountGUID")]
    pub account_guid: String,
    #[serde(rename = "userGUID")]
    pub user_guid: String,
    #[serde(rename = "isCompanyVerified")]
    pub is_company_verified: bool,
    #[serde(rename = "isAdminUser")]
    pub is_admin_user: bool,
    #[serde(rename = "isTestUser")]
    pub is_test_user: bool,
    #[serde(rename = "isInternalUser")]
    pub is_internal_user: bool,
    #[serde(rename = "isDespatchNoteIntegrated")]
    pub is_despatch_note_integrated: bool,
    #[serde(rename = "despatchNotePrintFormat")]
    pub despatch_note_print_format: String,
    #[serde(rename = "shouldGenerateCN")]
    pub should_generate_cn: bool,
    #[serde(rename = "shouldGenerateProofOfPostage")]
    pub should_generate_proof_of_postage: bool,
    #[serde(rename = "printFormat")]
    pub print_format: String,
    #[serde(rename = "labelsPerPage")]
    pub labels_per_page: i64,
    #[serde(rename = "futureDatedOrdersEnabled")]
    pub future_dated_orders_enabled: bool,
    #[serde(rename = "isAccountSuspended")]
    pub is_account_suspended: bool,
    #[serde(rename = "accountVerified")]
    pub account_verified: bool,
    #[serde(rename = "royalMailServices")]
    pub royal_mail_services: Vec<RoyalMailService>,
    pub departments: Vec<::serde_json::Value>,
    pub carriers: Vec<Carrier>,
    #[serde(rename = "accountName")]
    pub account_name: String,
    #[serde(rename = "accountFirstName")]
    pub account_first_name: String,
    #[serde(rename = "accountLastName")]
    pub account_last_name: String,
    #[serde(rename = "userFirstName")]
    pub user_first_name: String,
    #[serde(rename = "userSurname")]
    pub user_surname: String,
    #[serde(rename = "isCommunicationPreferenceSet")]
    pub is_communication_preference_set: bool,
    #[serde(rename = "isImpersonated")]
    pub is_impersonated: bool,
    pub features: Vec<::serde_json::Value>,
    #[serde(rename = "globalSettings")]
    pub global_settings: Vec<GlobalSetting>,
    #[serde(rename = "batchOrderCarrierLimits")]
    pub batch_order_carrier_limits: BatchOrderCarrierLimits,
    #[serde(rename = "currentCarrierType")]
    pub current_carrier_type: CurrentCarrierType,
    #[serde(rename = "channelShippingMethods")]
    pub channel_shipping_methods: ChannelShippingMethods,
    #[serde(rename = "futureDespatchDateMaxDays")]
    pub future_despatch_date_max_days: i64,
    #[serde(rename = "printingSettings")]
    pub printing_settings: ::serde_json::Value,
    #[serde(rename = "bankHolidays")]
    pub bank_holidays: BankHolidays,
    #[serde(rename = "maxPackageWeightKgDomestic")]
    pub max_package_weight_kg_domestic: f64,
    #[serde(rename = "maxPackageWeightKgInternational")]
    pub max_package_weight_kg_international: f64,
    #[serde(rename = "authServerChangePasswordUrl")]
    pub auth_server_change_password_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RoyalMailService {
    #[serde(rename = "shippingServiceID")]
    pub shipping_service_id: i64,
    #[serde(rename = "carrierID")]
    pub carrier_id: i64,
    #[serde(rename = "carrierShippingServiceID")]
    pub carrier_shipping_service_id: i64,
    #[serde(rename = "carrierTypeID")]
    pub carrier_type_id: i64,
    #[serde(rename = "externalServiceID")]
    pub external_service_id: i64,
    #[serde(rename = "serviceCode")]
    pub service_code: String,
    #[serde(rename = "serviceName")]
    pub service_name: String,
    #[serde(rename = "maxCompensationLevel")]
    pub max_compensation_level: i64,
    #[serde(rename = "isSignedFor")]
    pub is_signed_for: bool,
    #[serde(rename = "allowsSigning")]
    pub allows_signing: bool,
    #[serde(rename = "allowsSMSNotification")]
    pub allows_smsnotification: bool,
    #[serde(rename = "allowsEmailNotification")]
    pub allows_email_notification: bool,
    #[serde(rename = "allowsDeliveryDutyPaid")]
    pub allows_delivery_duty_paid: bool,
    #[serde(rename = "isLBT")]
    pub is_lbt: bool,
    #[serde(rename = "isHighVolume")]
    pub is_high_volume: bool,
    #[serde(rename = "allowsSaturdayGuaranteed")]
    pub allows_saturday_guaranteed: bool,
    #[serde(rename = "allowsLocalCollect")]
    pub allows_local_collect: bool,
    #[serde(rename = "requiresContractNumber")]
    pub requires_contract_number: bool,
    #[serde(rename = "contractNumber")]
    pub contract_number: ::serde_json::Value,
    #[serde(rename = "consequentialLossAmounts")]
    pub consequential_loss_amounts: ::serde_json::Value,
    #[serde(rename = "serviceEnhancement")]
    pub service_enhancement: ServiceEnhancement,
    #[serde(rename = "serviceLevel")]
    pub service_level: ServiceLevel,
    #[serde(rename = "validPackagings")]
    pub valid_packagings: Vec<ValidPackaging>,
    #[serde(rename = "validCountries")]
    pub valid_countries: Vec<ValidCountry>,
    #[serde(rename = "unsupportedPostingFromCountries")]
    pub unsupported_posting_from_countries: Vec<i64>,
    #[serde(rename = "isFavourite")]
    pub is_favourite: bool,
    #[serde(rename = "isHidden")]
    pub is_hidden: bool,
    #[serde(rename = "isReturn")]
    pub is_return: bool,
    #[serde(rename = "allowsSafePlace")]
    pub allows_safe_place: bool,
    pub aliases: Vec<::serde_json::Value>,
    #[serde(rename = "serviceRegisterCode")]
    pub service_register_code: ::serde_json::Value,
    #[serde(rename = "isIncludeTrackedReturn")]
    pub is_include_tracked_return: bool,
    #[serde(rename = "trackedReturnCarrierShippingServiceID")]
    pub tracked_return_carrier_shipping_service_id: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ServiceEnhancement {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ServiceLevel {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ValidPackaging {
    pub id: i64,
    pub name: String,
    #[serde(rename = "maxWeightG")]
    pub max_weight_g: i64,
    #[serde(rename = "minWeightG")]
    pub min_weight_g: i64,
    pub flag: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ValidCountry {
    pub id: i64,
    pub packagings: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Carrier {
    #[serde(rename = "carrierID")]
    pub carrier_id: i64,
    #[serde(rename = "carrierTypeID")]
    pub carrier_type_id: i64,
    pub name: String,
    #[serde(rename = "isCurrent")]
    pub is_current: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GlobalSetting {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BatchOrderCarrierLimits {
    #[serde(rename = "2")]
    pub n2: i64,
    #[serde(rename = "1")]
    pub n1: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CurrentCarrierType {
    #[serde(rename = "carrierTypeID")]
    pub carrier_type_id: i64,
    #[serde(rename = "requiresManifesting")]
    pub requires_manifesting: bool,
    #[serde(rename = "requiresPayment")]
    pub requires_payment: bool,
    #[serde(rename = "isDeliveryDutyPaidActive")]
    pub is_delivery_duty_paid_active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ChannelShippingMethods {
    pub ebay: Vec<Ebay>,
    pub amazon: Vec<Amazon>,
    pub etsy: Vec<Etsy>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Ebay {
    pub name: String,
    pub value: String,
    pub group: Option<String>,
    #[serde(rename = "accountID")]
    pub account_id: Option<i64>,
    #[serde(rename = "channelShippingMethodID")]
    pub channel_shipping_method_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Amazon {
    pub name: String,
    pub value: String,
    pub group: ::serde_json::Value,
    #[serde(rename = "accountID")]
    pub account_id: ::serde_json::Value,
    #[serde(rename = "channelShippingMethodID")]
    pub channel_shipping_method_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Etsy {
    pub name: String,
    pub value: String,
    pub group: ::serde_json::Value,
    #[serde(rename = "accountID")]
    pub account_id: i64,
    #[serde(rename = "channelShippingMethodID")]
    pub channel_shipping_method_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BankHolidays {
    #[serde(rename = "2020-01-01")]
    pub n20200101: String,
    #[serde(rename = "2020-01-02")]
    pub n20200102: String,
    #[serde(rename = "2020-03-17")]
    pub n20200317: String,
    #[serde(rename = "2020-04-10")]
    pub n20200410: String,
    #[serde(rename = "2020-04-13")]
    pub n20200413: String,
    #[serde(rename = "2020-05-08")]
    pub n20200508: String,
    #[serde(rename = "2020-05-25")]
    pub n20200525: String,
    #[serde(rename = "2020-07-13")]
    pub n20200713: String,
    #[serde(rename = "2020-08-03")]
    pub n20200803: String,
    #[serde(rename = "2020-08-31")]
    pub n20200831: String,
    #[serde(rename = "2020-11-30")]
    pub n20201130: String,
    #[serde(rename = "2020-12-25")]
    pub n20201225: String,
    #[serde(rename = "2020-12-28")]
    pub n20201228: String,
}

// RM Orders
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct RMOrders {
    #[serde(rename = "hintVisibility")]
    pub hint_visibility: HintVisibility,
    pub data: Vec<Daum>,
    pub paging: Paging,
    #[serde(rename = "invalidFilterColumns")]
    pub invalid_filter_columns: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HintVisibility {
    #[serde(rename = "createOrder")]
    pub create_order: bool,
    #[serde(rename = "applyPostage")]
    pub apply_postage: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Daum {
    #[serde(rename = "orderGUID")]
    pub order_guid: String,
    #[serde(rename = "accountOrderNumber")]
    pub account_order_number: i64,
    #[serde(rename = "channelName")]
    pub channel_name: String,
    #[serde(rename = "channelTypeCssClass")]
    pub channel_type_css_class: String,
    #[serde(rename = "companyIdentityName")]
    pub company_identity_name: String,
    #[serde(rename = "channelOrderRef")]
    pub channel_order_ref: String,
    #[serde(rename = "orderDate")]
    pub order_date: String,
    #[serde(rename = "despatchDate")]
    pub despatch_date: ::serde_json::Value,
    #[serde(rename = "accountBatchNumber")]
    pub account_batch_number: String,
    #[serde(rename = "accountManifestNumber")]
    pub account_manifest_number: String,
    #[serde(rename = "batchGUID")]
    pub batch_guid: String,
    #[serde(rename = "manifestGUID")]
    pub manifest_guid: String,
    pub products: Vec<Product>,
    #[serde(rename = "customerName")]
    pub customer_name: String,
    pub email: ::serde_json::Value,
    pub postcode: String,
    #[serde(rename = "countryID")]
    pub country_id: i64,
    #[serde(rename = "isInternational")]
    pub is_international: bool,
    #[serde(rename = "hasCN23Form")]
    pub has_cn23_form: bool,
    #[serde(rename = "orderTotal")]
    pub order_total: f64,
    #[serde(rename = "exchangeRate")]
    pub exchange_rate: f64,
    #[serde(rename = "orderWeight")]
    pub order_weight: f64,
    #[serde(rename = "carrierShippingServiceID")]
    pub carrier_shipping_service_id: Option<i64>,
    #[serde(rename = "shippingService")]
    pub shipping_service: Option<String>,
    #[serde(rename = "shippingServiceCode")]
    pub shipping_service_code: Option<String>,
    #[serde(rename = "specialInstructions")]
    pub special_instructions: Option<String>,
    pub status: String,
    #[serde(rename = "currencyCode")]
    pub currency_code: String,
    #[serde(rename = "currencySymbol")]
    pub currency_symbol: String,
    #[serde(rename = "currencyID")]
    pub currency_id: i64,
    #[serde(rename = "channelCountryTwoLetterISOCode")]
    pub channel_country_two_letter_isocode: String,
    #[serde(rename = "channelCountryName")]
    pub channel_country_name: String,
    #[serde(rename = "postagePaymentStatus")]
    pub postage_payment_status: String,
    #[serde(rename = "refundRequestStatus")]
    pub refund_request_status: String,
    #[serde(rename = "trackingOrConfirmationNumber")]
    pub tracking_or_confirmation_number: ::serde_json::Value,
    #[serde(rename = "shippingTrackingStatus")]
    pub shipping_tracking_status: ::serde_json::Value,
    #[serde(rename = "isTracked")]
    pub is_tracked: bool,
    #[serde(rename = "postcodeValid")]
    pub postcode_valid: bool,
    #[serde(rename = "channelShippingMethod")]
    pub channel_shipping_method: String,
    #[serde(rename = "channelShippingCost")]
    pub channel_shipping_cost: f64,
    #[serde(rename = "isShippingAddressValid")]
    pub is_shipping_address_valid: bool,
    #[serde(rename = "despatchOrManifestDate")]
    pub despatch_or_manifest_date: ::serde_json::Value,
    #[serde(rename = "isManifestDate")]
    pub is_manifest_date: bool,
    #[serde(rename = "isFutureDespatchDate")]
    pub is_future_despatch_date: bool,
    #[serde(rename = "isAnonymised")]
    pub is_anonymised: bool,
    #[serde(rename = "channelIsDeleted")]
    pub channel_is_deleted: bool,
    #[serde(rename = "isElectronicCustomsRequired")]
    pub is_electronic_customs_required: bool,
    #[serde(rename = "hasRequiredCustomsInfo")]
    pub has_required_customs_info: bool,
    #[serde(rename = "receiveSmsNotification")]
    pub receive_sms_notification: bool,
    #[serde(rename = "receiveEmailNotification")]
    pub receive_email_notification: bool,
    #[serde(rename = "requestSignatureUponDelivery")]
    pub request_signature_upon_delivery: bool,
    #[serde(rename = "guaranteedSaturdayDelivery")]
    pub guaranteed_saturday_delivery: bool,
    #[serde(rename = "isLocalCollect")]
    pub is_local_collect: bool,
    #[serde(rename = "isDeliveryDutyPaid")]
    pub is_delivery_duty_paid: bool,
    pub tags: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Product {
    pub name: String,
    pub sku: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Paging {
    #[serde(rename = "totalItems")]
    pub total_items: i64,
    #[serde(rename = "currentPageIndex")]
    pub current_page_index: i64,
    #[serde(rename = "maxPageIndex")]
    pub max_page_index: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

// Ebay

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EbayOrders {
    pub href: Option<String>,
    pub total: i64,
    pub next: Option<String>,
    pub limit: i64,
    pub offset: i64,
    pub orders: Vec<EbayOrder>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct EbayOrder {
    #[serde(rename = "orderId")]
    pub order_id: String,
    #[serde(rename = "legacyOrderId")]
    pub legacy_order_id: String,
    #[serde(rename = "creationDate")]
    pub creation_date: DateTime<Utc>,
    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: DateTime<Utc>,
    #[serde(rename = "orderFulfillmentStatus")]
    pub order_fulfillment_status: String,
    #[serde(rename = "orderPaymentStatus")]
    pub order_payment_status: String,
    #[serde(rename = "sellerId")]
    pub seller_id: String,
    pub buyer: Buyer,
    #[serde(rename = "pricingSummary")]
    pub pricing_summary: PricingSummary,
    #[serde(rename = "cancelStatus")]
    pub cancel_status: CancelStatus,
    #[serde(rename = "paymentSummary")]
    pub payment_summary: PaymentSummary,
    #[serde(rename = "fulfillmentStartInstructions")]
    pub fulfillment_start_instructions: Vec<FulfillmentStartInstruction>,
    #[serde(rename = "fulfillmentHrefs")]
    pub fulfillment_hrefs: Vec<::serde_json::Value>,
    #[serde(rename = "lineItems")]
    pub line_items: Vec<LineItem>,
    #[serde(rename = "salesRecordReference")]
    pub sales_record_reference: Option<String>,
    #[serde(rename = "totalFeeBasisAmount")]
    pub total_fee_basis_amount: TotalFeeBasisAmount,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Buyer {
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PricingSummary {
    #[serde(rename = "priceSubtotal")]
    pub price_subtotal: PriceSubtotal,
    #[serde(rename = "deliveryCost")]
    pub delivery_cost: DeliveryCost,
    pub total: Total,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PriceSubtotal {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DeliveryCost {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Total {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct CancelStatus {
    #[serde(rename = "cancelState")]
    pub cancel_state: String,
    #[serde(rename = "cancelRequests")]
    pub cancel_requests: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PaymentSummary {
    #[serde(rename = "totalDueSeller")]
    pub total_due_seller: Option<TotalDueSeller>,
    pub refunds: Vec<::serde_json::Value>,
    pub payments: Vec<Payment>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TotalDueSeller {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Payment {
    #[serde(rename = "paymentMethod")]
    pub payment_method: String,
    #[serde(rename = "paymentReferenceId")]
    pub payment_reference_id: String,
    #[serde(rename = "paymentDate")]
    pub payment_date: Option<DateTime<Utc>>,
    pub amount: Amount,
    #[serde(rename = "paymentStatus")]
    pub payment_status: String,
    #[serde(rename = "paymentHolds")]
    pub payment_holds: Option<Vec<PaymentHold>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Amount {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PaymentHold {
    #[serde(rename = "holdReason")]
    pub hold_reason: Option<String>,
    #[serde(rename = "holdAmount")]
    pub hold_amount: HoldAmount,
    #[serde(rename = "holdState")]
    pub hold_state: String,
    #[serde(rename = "releaseDate")]
    pub release_date: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct HoldAmount {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct FulfillmentStartInstruction {
    #[serde(rename = "fulfillmentInstructionsType")]
    pub fulfillment_instructions_type: String,
    #[serde(rename = "minEstimatedDeliveryDate")]
    pub min_estimated_delivery_date: DateTime<Utc>,
    #[serde(rename = "maxEstimatedDeliveryDate")]
    pub max_estimated_delivery_date: DateTime<Utc>,
    #[serde(rename = "ebaySupportedFulfillment")]
    pub ebay_supported_fulfillment: Option<bool>,
    #[serde(rename = "shippingStep")]
    pub shipping_step: ShippingStep,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShippingStep {
    #[serde(rename = "shipTo")]
    pub ship_to: ShipTo,
    #[serde(rename = "shippingCarrierCode")]
    pub shipping_carrier_code: Option<String>,
    #[serde(rename = "shippingServiceCode")]
    pub shipping_service_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShipTo {
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    #[serde(rename = "contactAddress")]
    pub contact_address: ContactAddress,
    #[serde(rename = "primaryPhone")]
    pub primary_phone: PrimaryPhone,
    pub email: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ContactAddress {
    #[serde(rename = "addressLine1")]
    pub address_line1: Option<String>,
    #[serde(rename = "addressLine2")]
    pub address_line2: Option<String>,
    pub city: Option<String>,
    #[serde(rename = "stateOrProvince")]
    pub state_or_province: Option<String>,
    #[serde(rename = "postalCode")]
    pub postal_code: Option<String>,
    #[serde(rename = "countryCode")]
    pub country_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct PrimaryPhone {
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LineItem {
    #[serde(rename = "lineItemId")]
    pub line_item_id: String,
    #[serde(rename = "legacyItemId")]
    pub legacy_item_id: String,
    pub title: String,
    #[serde(rename = "lineItemCost")]
    pub line_item_cost: LineItemCost,
    pub quantity: i64,
    #[serde(rename = "soldFormat")]
    pub sold_format: String,
    #[serde(rename = "listingMarketplaceId")]
    pub listing_marketplace_id: String,
    #[serde(rename = "purchaseMarketplaceId")]
    pub purchase_marketplace_id: String,
    #[serde(rename = "lineItemFulfillmentStatus")]
    pub line_item_fulfillment_status: String,
    pub total: Total2,
    #[serde(rename = "deliveryCost")]
    pub delivery_cost: DeliveryCost2,
    #[serde(rename = "appliedPromotions")]
    pub applied_promotions: Vec<::serde_json::Value>,
    pub taxes: Vec<::serde_json::Value>,
    pub properties: Option<Properties>,
    #[serde(rename = "lineItemFulfillmentInstructions")]
    pub line_item_fulfillment_instructions: LineItemFulfillmentInstructions,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LineItemCost {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Total2 {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DeliveryCost2 {
    #[serde(rename = "shippingCost")]
    pub shipping_cost: ShippingCost,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ShippingCost {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Properties {
    #[serde(rename = "buyerProtection")]
    pub buyer_protection: Option<bool>,
    #[serde(rename = "soldViaAdCampaign")]
    pub sold_via_ad_campaign: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct LineItemFulfillmentInstructions {
    #[serde(rename = "minEstimatedDeliveryDate")]
    pub min_estimated_delivery_date: DateTime<Utc>,
    #[serde(rename = "maxEstimatedDeliveryDate")]
    pub max_estimated_delivery_date: DateTime<Utc>,
    #[serde(rename = "shipByDate")]
    pub ship_by_date: Option<DateTime<Utc>>,
    #[serde(rename = "guaranteedDelivery")]
    pub guaranteed_delivery: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct TotalFeeBasisAmount {
    pub value: String,
    pub currency: String,
    #[serde(rename = "convertedFromValue")]
    pub converted_from_value: Option<String>,
    #[serde(rename = "convertedFromCurrency")]
    pub converted_from_currency: Option<String>,
}

// =============================================== OnBuy =====================================================

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OnBuyOrders {
    pub results: Vec<OnBuyOrder>,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OnBuyOrder {
    pub order_id: String,
    pub onbuy_internal_reference: i64,
    pub date: String,
    pub updated_at: String,
    pub cancelled_at: ::serde_json::Value,
    pub shipped_at: String,
    pub status: String,
    pub site_id: i64,
    pub site_name: String,
    pub price_subtotal: String,
    pub price_delivery: String,
    pub price_total: String,
    pub price_discount: String,
    #[serde(rename = "sales_fee_ex_VAT")]
    pub sales_fee_ex_vat: String,
    #[serde(rename = "sales_fee_inc_VAT")]
    pub sales_fee_inc_vat: String,
    pub currency_code: String,
    pub dispatched: bool,
    pub delivery_service: String,
    pub delivery_tag: String,
    pub stripe_transaction_id: ::serde_json::Value,
    pub paypal_capture_id: String,
    pub buyer: OnBuyBuyer,
    pub billing_address: BillingAddress,
    pub delivery_address: DeliveryAddress,
    pub fee: Fee,
    pub products: Vec<OnBuyProduct>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OnBuyBuyer {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub ip_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct BillingAddress {
    pub name: ::serde_json::Value,
    #[serde(rename = "line_1")]
    pub line1: String,
    #[serde(rename = "line_2")]
    pub line2: String,
    #[serde(rename = "line_3")]
    pub line3: ::serde_json::Value,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub country: String,
    pub country_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct DeliveryAddress {
    pub name: String,
    #[serde(rename = "line_1")]
    pub line1: String,
    #[serde(rename = "line_2")]
    pub line2: String,
    #[serde(rename = "line_3")]
    pub line3: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub country: String,
    pub country_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Fee {
    pub boost_marketing_fee_excluding_vat: String,
    pub category_fee_excluding_vat: String,
    pub delivery_fee_excluding_vat: String,
    pub total_fee_excluding_vat: String,
    pub vat_rate: String,
    pub total_fee_including_vat: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct OnBuyProduct {
    pub onbuy_internal_reference: i64,
    pub name: String,
    pub sku: String,
    pub condition: String,
    pub condition_id: i64,
    pub quantity: i64,
    pub quantity_dispatched: i64,
    pub unit_price: String,
    pub total_price: String,
    pub expected_dispatch_date: String,
    pub expected_delivery_date: String,
    pub opc: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Metadata {
    pub limit: i64,
    pub offset: i64,
    pub total_rows: i64,
    pub filters: Filters,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Filters {
    pub status: String,
}
