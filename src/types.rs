use std::collections::HashMap;
use std::str::FromStr;

// use ethers::prelude::LocalWallet;
// use ethers::types::{Address, H160, H256};
// use ethers_providers::{Http, Provider};
use ethers::prelude::*;

use crate::constants::ItemType;
use crate::constants::OrderType;

// This file contains the types(Structures of data types)
// Why did I use Lazy<Address> over Address? I have no technical reason
// I saw this pattern in gakonst's opensea-rs
// Made everything here pub so I could use them in other files

// TODO  I have to make sure each type is ideal at some point
// I used u64 for every field that was a number, I'm not sure if this was a good idea
// FIXME start_amount and end_amount are Strings, I'll have to work on that later
// FIXME recipient should probably be an Address or Lazy<Address>

#[derive(Debug, PartialEq)]
pub struct Overrides {
    pub contract_address: H160,
    pub default_conduitkey: H256,
}

#[derive(Debug, PartialEq)]
pub struct SeaportConfig {
    pub ascending_amount_fulfillment_buffer: Option<u64>,
    pub balance_and_approval_checks_on_order_creation: Option<bool>,
    pub conduit_key_to_conduit: Option<HashMap<H256, H160>>,
    pub overides: Option<Overrides>,
}

impl SeaportConfig {
    // I faced issues setting the conduit key to a constanst, so I'm using a function for it instead - Got the idea from asnared :)
    // I might move this to SeaportConfig
    pub fn get_opensea_conduit_key() -> H256 {
        H256::from_str("0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000")
            .unwrap()
    }
}

#[derive(Debug)]
pub struct OfferItem {
    pub item_type: ItemType,
    pub token: Address,
    pub identifier_or_criteria: String,
    pub start_amount: String,
    pub end_amount: String,
}

#[derive(Debug)]
pub struct ConsiderationItem {
    pub item_type: ItemType,
    pub token: String,
    pub identifier_or_criteria: String,
    pub start_amount: String,
    pub end_amount: String,
    pub recipient: String,
}

// FIXME TS UNION TYPES ARE MORE LIKE ENUMS THAN STRUCTS
pub struct Item {
    pub offer_item: OfferItem,
    pub consideration_item: ConsiderationItem,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct OrderComponents {
    pub order_parameters: OrderParameters,
    pub counter: u64,
}

#[derive(Debug)]
pub struct Order {
    pub parameters: OrderParameters,
    pub signature: H256,
}

// I needed a way to set provider or signer to just two types, thus the enum
#[derive(Debug)]
pub enum ProviderOrSigner {
    Provider(Provider<Http>),
    Signer(LocalWallet),
}

#[derive(Debug)]
pub struct BasicErc721Item {
    pub item_type: ItemType,
    // FIXME This should be an address
    pub token: String,
    pub identifier: String,
}

#[derive(Debug)]
pub struct Erc721ItemWithCriteria {
    item_type: ItemType,
    // FIXME This should be an address
    token: String,
    identifiers: Vec<String>,
    // FIXME This should be a number
    amount: Option<String>,
    // FIXME THIS SHOULD BE A NUMBER
    end_amount: Option<String>,
}

#[derive(Debug)]
pub enum Erc721Item {
    BasicErc721Item(BasicErc721Item),
    Erc721ItemWithCriteria(Erc721ItemWithCriteria),
}

#[derive(Debug)]
pub struct BasicErc1155Item {
    item_type: ItemType,
    // FIXME SHOULD BE ADDRESS
    token: String,
    identifier: String,
    amount: String,
    end_amount: Option<String>,
}

#[derive(Debug)]
pub struct Erc1155ItemWithCriteria {
    item_type: ItemType,
    token: String,
    identifiers: Vec<String>,
    amount: String,
    end_amount: Option<String>,
}

#[derive(Debug)]
pub enum Erc1155Item {
    BasicErc1155Item(BasicErc1155Item),
    Erc1155ItemWithCriteria(Erc1155ItemWithCriteria),
}

#[derive(Debug)]
pub struct CurrencyItem {
    pub token: Option<String>,
    pub amount: String,
    pub end_amount: Option<String>,
}

#[derive(Debug)]
pub enum CreateInputItem {
    Erc721Item(Erc721Item),
    Erc1155Item(Erc1155Item),
    CurrencyItem(CurrencyItem),
}

#[derive(Debug)]
pub struct ConsiderationInputItem {
    pub create_input_item: CreateInputItem,
    pub recipient: Option<String>,
}

#[derive(Debug)]
pub struct Fee {
    recipient: String,
    basis_points: u32,
}

#[derive(Debug)]
pub struct CreateOrderInput {
    pub conduit_key: Option<Address>,
    pub zone: Option<Address>,
    pub start_time: Option<U256>,
    pub end_time: Option<U256>,
    pub offer: Vec<CreateInputItem>,
    pub consideration: Vec<ConsiderationInputItem>,
    pub counter: Option<u32>,
    pub fees: Option<Vec<Fee>>,
    pub allow_partial_fills: Option<bool>,
    pub restricted_by_zone: Option<bool>,
    pub use_proxy: Option<bool>,
    pub domain: Option<String>,
    // Will probably be a bytes type
    pub salt: Option<String>,
}
