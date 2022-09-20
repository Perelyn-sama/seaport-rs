use crate::constants::ItemType;
use crate::constants::OrderType;
use ethers_providers::Provider;
use ethers::{
    types::{Address, H256},
};
use std::collections::HashMap;
use ethers::prelude::Lazy;

// This file contains the types(Structures of data types)
// Why did I use Lazy<Address> over Address? I have no technical reason
// I saw this pattern in gakonst's opensea-rs
// Made everything here pub so I could use them in other files

// TODO  I have to make sure each type is ideal at some point
// I used u64 for every field that was a number, I'm not sure if this was a good idea
// FIXME start_amount and end_amount are Strings, I'll have to work on that later
// FIXME recipient should probably be an Address or Lazy<Address>

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
    pub signature: H256,
}

// I needed a way to set provider or signer to just two types, thus the enum
pub enum ProviderOrSigner {
    Provider(Provider<P>),
    Signer(String),
}