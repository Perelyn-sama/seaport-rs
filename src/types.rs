// use crate::{constants, contracts};
use crate::constants::ItemType;
use crate::constants::OrderType;

use ethers::{
    types::{Address, H256},
};
// use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Overrides {
    contractAddress: Address,
    defaultConduitkey: H256,
}

pub struct SeaportConfig {
    ascendingAmountFulfillmentBuffer: Option<u64>,
    balanceAndApprovalChecksOnOrderCreation: Option<bool>,
    conduitKeyToConduit: Option<HashMap<Address, Address>>,
    overides: Option<Overrides>,
}

pub struct OfferItem {
    item_type: ItemType,
    token: Address,
    identifier_or_criteria: String,
    start_amount: String,
    end_amount: String,
}

pub struct ConsiderationItem {
    item_type: ItemType,
    token: String,
    identifier_or_criteria: String,
    start_amount: String,
    end_amount: String,
    recipient: String,
}

pub union Item {
    OfferItem: OfferItem,
    ConsiderationItem: ConsiderationItem,
}

pub struct OrderParameters {
    offerer: Address,
    zone: String,
    order_type: OrderType,
    start_time: u64,
    end_time: u64,
    zone_hash: String,
    salt: String,
    // Using a vec here but would an array be better?
    // const xs: [OfferItem; 1];
    offer: Vec<OfferItem>,
    consideration: Vec<ConsiderationItem>,
    total_original_consideration_items: u64,
    conduit_key: H256,
}

pub struct OrderComponents {
    OrderParameters: OrderParameters,
    counter: u64,
}

pub struct Order {
    parameters: OrderParameters,
    // This should be turned to a H256 or somethig
    signature: H256,
}
