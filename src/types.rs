use crate::constants::{ItemType, OrderType};
use ethers::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Overrides {
    pub contract_address: H160,
    pub default_conduitkey: H256,
}

#[derive(Debug, PartialEq, Eq)]
pub struct SeaportConfig {
    pub ascending_amount_fulfillment_buffer: Option<u64>,
    pub balance_and_approval_checks_on_order_creation: Option<bool>,
    pub conduit_key_to_conduit: Option<HashMap<H256, H160>>,
    pub overides: Option<Overrides>,
}

impl SeaportConfig {
    pub fn get_opensea_conduit_key() -> H256 {
        H256::from_str("0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000")
            .unwrap()
    }
}

#[derive(Debug)]
pub struct OfferItem {
    pub item_type: ItemType,
    pub token: H160,
    pub identifier_or_criteria: String,
    pub start_amount: U256,
    pub end_amount: U256,
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
    pub zone: Address,
    pub order_type: OrderType,
    pub start_time: U256,
    pub end_time: U256,
    pub zone_hash: String,
    pub salt: String,
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

#[derive(Debug)]
pub enum ProviderOrSigner {
    Provider(Provider<Http>),
    Signer(SignerMiddleware<Provider<Http>, LocalWallet>),
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
    pub item_type: ItemType,
    // FIXME This should be an address
    pub token: String,
    pub identifiers: Vec<String>,
    // FIXME This should be a number
    pub amount: Option<String>,
    // FIXME THIS SHOULD BE A NUMBER
    pub end_amount: Option<String>,
}

#[derive(Debug)]
pub enum Erc721Item {
    BasicErc721Item(BasicErc721Item),
    Erc721ItemWithCriteria(Erc721ItemWithCriteria),
}

#[derive(Debug)]
pub struct BasicErc1155Item {
    pub item_type: ItemType,
    // FIXME SHOULD BE ADDRESS
    pub token: String,
    pub identifier: String,
    pub amount: String,
    pub end_amount: Option<String>,
}

#[derive(Debug)]
pub struct Erc1155ItemWithCriteria {
    pub item_type: ItemType,
    pub token: String,
    pub identifiers: Vec<String>,
    pub amount: String,
    pub end_amount: Option<String>,
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
    pub recipient: String,
    pub basis_points: u32,
}

#[derive(Debug)]
pub struct CreateOrderInput {
    pub conduit_key: Option<H256>,
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
