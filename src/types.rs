// use crate::{constants, contracts};
use crate::constants::ItemType;
use crate::constants::OrderType;

use ethers::{
    types::{Address, H256},
};
// use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ethers::prelude::Lazy;

pub struct Overrides {
    pub contractAddress: Lazy<Address>,
    pub defaultConduitkey: H256,
}

pub struct SeaportConfig {
    pub ascendingAmountFulfillmentBuffer: Option<u64>,
    pub balanceAndApprovalChecksOnOrderCreation: Option<bool>,
    pub conduitKeyToConduit: Option<HashMap<H256, Lazy<Address>>>,
    pub overides: Option<Overrides>,
}

pub struct OfferItem {
    pub item_type: ItemType,
    pub token: Address,
    pub identifier_or_criteria: String,
    pub start_amount: String,
    pub end_amount: String,
}

pub struct ConsiderationItem {
    pub item_type: ItemType,
    pub token: String,
    pub identifier_or_criteria: String,
    pub start_amount: String,
    pub end_amount: String,
    pub recipient: String,
}

pub struct Item {
    pub OfferItem: OfferItem,
    pub ConsiderationItem: ConsiderationItem,
}

pub struct OrderParameters {
    pub offerer: Address,
    pub zone: String,
    pub order_type: OrderType,
    pub start_time: u64,
    pub end_time: u64,
    pub zone_hash: String,
    pub salt: String,
    // Using a vec here but would an array be better?
    // const xs: [OfferItem; 1];
    pub offer: Vec<OfferItem>,
    pub consideration: Vec<ConsiderationItem>,
    pub total_original_consideration_items: u64,
    pub conduit_key: H256,
}

pub struct OrderComponents {
    pub OrderParameters: OrderParameters,
    pub counter: u64,
}

pub struct Order {
    pub parameters: OrderParameters,
    // This should be turned to a H256 or somethig
    pub signature: H256,
}
