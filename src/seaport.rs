use crate::bindings::seaport::seaport::seaport;
use crate::constants;
use crate::constants::ItemType;
use crate::types::{
    BasicErc721Item, ConsiderationInputItem, ConsiderationItem, CreateInputItem, CreateOrderInput,
    CurrencyItem, Erc721Item, OfferItem, OrderParameters, Overrides,
    SeaportConfig,
};
use ethers::prelude::{builders::ContractCall, *};
use ethers::types::Address;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;

#[derive(Debug)]
pub struct Seaport<M> {
    /// The seaport contract.
    pub contract: seaport<M>,
    /// Config for the seaport sdk
    pub seaport_config: Option<SeaportConfig>,
}

impl<M> Seaport<M> {
    /// Returns a reference to the seaport contract.
    pub fn contract(&self) -> &seaport<M> {
        &self.contract
    }
}

impl<M: Middleware> Seaport<M> {
    /// Creates a new instance from using the provided address.
    pub fn new(client: Arc<M>, address: Address, cfg: SeaportConfig) -> Self {
        let contract = seaport::new(address, client);
        match cfg.ascending_amount_fulfillment_buffer {
            Some(_x) => Self {
                contract,
                seaport_config: Some(cfg),
            },

            None => Self {
                contract,
                seaport_config: Some(SeaportConfig::default()),
            },
        }
    }
    pub fn counter(&self, offerer: Address) -> ContractCall<M, U256> {
        let seaport = self.contract();
        let counter = seaport.get_counter(offerer);
        counter
    }

    // making it work
    // I need to
    pub fn create_order(
        &self,
        param: CreateOrderInput,
        account_address: Option<Address>,
    ) -> OrderParameters {

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let token_address = "0xf5de760f2e916647fd766B4AD9E85ff943cE3A2b"
            .parse::<Address>()
            .unwrap();

        let offer_item = OfferItem {
            item_type: ItemType::ERC721,
            token: token_address,
            identifier_or_criteria: String::from("1672186"),
            start_amount: U256::from(start_time),
            end_amount: U256::max_value(),
        };

        let consideration_input = ConsiderationItem {
            item_type: ItemType::ERC721,
            token: "address".to_string(),
            identifier_or_criteria: String::from("0"),
            start_amount: String::from("10000000000000"),
            end_amount: String::from("10000000000000"),
            recipient: String::from("0x3C58dC9864e73aE2Ec9E0B11e00F786352A80F51"),
        };

        // let counter = &self.counter();
        // FIXME MORE TO CREATEORDERINPUT default function
        let _counter = 0.to_string();

        let consideration_vec = vec![consideration_input];
        let consideration_len = consideration_vec.len();

            // if let x = y {
            //     foo();
            // } else {
            //     bar();
            // }

        //I can extract data for what I need here form the the othe place
        //i do not need to make it here agian
        OrderParameters {
            offerer: account_address.unwrap(),
            zone: param.zone.unwrap(),
            order_type: constants::OrderType::FullOpen,
            start_time: param.start_time.unwrap(),
            end_time: param.end_time.unwrap(),
            // FIXME REMOVE HARD CODED VALUE
            // FIXME USE format!("{:?}", "0".as_bytes());
            zone_hash: "48".to_string(),
            // FIXME REMOVE HARDCODE VERSION
            // salt: param.salt.unwrap(),
            salt: "".to_string(),
            offer: vec![offer_item],
            consideration: consideration_vec,
            total_original_consideration_items: consideration_len as u64,
            conduit_key: param.conduit_key.unwrap(),
        }
    }

    pub fn get_counter(&self, offerer: Address) -> ContractCall<M, U256> {
        let seaport = self.contract();
        let counter = seaport.get_counter(offerer);
        counter
    }
}

impl<M> std::ops::Deref for Seaport<M> {
    type Target = seaport<M>;

    fn deref(&self) -> &Self::Target {
        self.contract()
    }
}
impl Default for SeaportConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            Self::get_opensea_conduit_key(),
            *constants::OPENSEA_CONDUIT_ADDRESS,
        );

        Self {
            ascending_amount_fulfillment_buffer: Some(300),
            balance_and_approval_checks_on_order_creation: Some(true),
            conduit_key_to_conduit: Some(map),
            overides: Some(Overrides {
                contract_address: *constants::OPENSEA_CONDUIT_ADDRESS,
                default_conduitkey: Self::get_opensea_conduit_key(),
            }),
        }
    }
}

impl Default for CreateOrderInput {
    fn default() -> Self {
        let offerer = String::from("0x3C58dC9864e73aE2Ec9E0B11e00F786352A80F51");

        let basic_erc721_item = BasicErc721Item {
            item_type: ItemType::ERC721,
            token: String::from("0xf5de760f2e916647fd766B4AD9E85ff943cE3A2b"),
            identifier: String::from("1672186"),
        };

        let erc721_item = Erc721Item::BasicErc721Item(basic_erc721_item);

        let offer_input_item = CreateInputItem::Erc721Item(erc721_item);

        let currency_item = CurrencyItem {
            token: None,
            amount: String::from("0.0001"),
            end_amount: None,
        };

        let currency_input_item = CreateInputItem::CurrencyItem(currency_item);

        let consideration_input_item = ConsiderationInputItem {
            create_input_item: currency_input_item,
            recipient: Some(offerer),
        };
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Self {
            // conduit_key: (*constants::OPENSEA_CONDUIT_KEY),
            conduit_key: Some(H256::from_str("0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000").unwrap()),
            zone: Some(*constants::ZERO_ADDRESS),
            start_time: Some(U256::from(start_time)),
            end_time: Some(U256::max_value()),
            offer: vec![offer_input_item],
            consideration: vec![consideration_input_item],
            counter: None,
            fees: None,
            allow_partial_fills: None,
            restricted_by_zone: None,
            use_proxy: None,
            domain: None,
            salt: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::CROSS_CHAIN_SEAPORT_ADDRESS;
    use crate::types::SeaportConfig;
    use ethers::prelude::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_seaport_config_default() {
        let mut map = HashMap::new();
        map.insert(
            SeaportConfig::get_opensea_conduit_key(),
            *constants::OPENSEA_CONDUIT_ADDRESS,
        );

        let cfg = SeaportConfig::default();
        let output = cfg;

        assert_eq!(output.ascending_amount_fulfillment_buffer, Some(300));
        assert_eq!(output.ascending_amount_fulfillment_buffer.unwrap(), 300);
        assert!(output
            .balance_and_approval_checks_on_order_creation
            .unwrap());
        assert_eq!(output.conduit_key_to_conduit.unwrap(), map);
        // assert_eq!(output.overides.contract_address.unwrap(), *constants::OPENSEA_CONDUIT_ADDRESS);
        // assert_eq!(output.overides.default_conduitkey.unwrap(), SeaportConfig::get_opensea_conduit_key());
    }

    #[tokio::test]
    async fn test_seaport_new() {
        let mut map = HashMap::new();
        map.insert(
            SeaportConfig::get_opensea_conduit_key(),
            *constants::OPENSEA_CONDUIT_ADDRESS,
        );

        let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
        let signer = LocalWallet::new(&mut rand::thread_rng());
        let signer_miidleware = SignerMiddleware::new(provider, signer);

        let client = Arc::new(signer_miidleware);

        let cfg = SeaportConfig::default();
        let cfg2 = SeaportConfig::default();

        let seaport = Seaport::new(
            client,
            Address::from_str(CROSS_CHAIN_SEAPORT_ADDRESS).unwrap(),
            cfg,
        );

        assert_eq!(seaport.seaport_config.unwrap(), cfg2);
    }
}
