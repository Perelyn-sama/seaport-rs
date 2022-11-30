use crate::bindings::seaport::seaport::seaport;
use crate::bindings::seaport::{
    ConsiderationItem, OfferItem, Order, OrderComponents, OrderParameters,
};
use crate::constants;
use crate::constants::ItemType;
use crate::types::{
    BasicErc721Item, ConsiderationInputItem, CreateInputItem, CreateOrderInput, CurrencyItem,
    Erc721Item, Overrides, SeaportConfig,
};
use ethers::prelude::{builders::ContractCall, *};
use ethers::types::Address;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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
            item_type: ItemType::ERC721 as u8,
            token: token_address,
            identifier_or_criteria: U256::from_str("1672186").unwrap(),
            start_amount: U256::from(start_time),
            end_amount: U256::max_value(),
        };

        let consideration_input = ConsiderationItem {
            item_type: ItemType::ERC721 as u8,
            token: token_address,
            identifier_or_criteria: String::from("0").parse().unwrap(),
            start_amount: String::from("10000000000000").parse().unwrap(),
            end_amount: String::from("10000000000000").parse().unwrap(),
            recipient: String::from("0x3C58dC9864e73aE2Ec9E0B11e00F786352A80F51")
                .parse()
                .unwrap(),
        };

        // let counter = &self.counter();
        // FIXME MORE TO CREATEORDERINPUT default function
        let _counter = 0.to_string();

        let consideration_vec = vec![consideration_input];
        let consideration_len = consideration_vec.len();

        OrderParameters {
            offerer: account_address.unwrap(),
            zone: param.zone.unwrap(),
            order_type: constants::OrderType::FullOpen as u8,
            start_time: param.start_time.unwrap(),
            end_time: param.end_time.unwrap(),
            // FIXME REMOVE HARD CODED VALUE
            // FIXME USE format!("{:?}", "0".as_bytes());
            zone_hash: <[u8; 32]>::from(param.conduit_key.unwrap()),
            // zone_hash: "48".to_string() ,
            // FIXME REMOVE HARDCODE VERSION
            // salt: param.salt.unwrap(),
            salt: "".to_string().parse().unwrap(),
            offer: vec![offer_item],
            consideration: consideration_vec,
            total_original_consideration_items: U256::from(consideration_len as u64),
            // conduit_key: param.conduit_key.unwrap(),
            conduit_key: <[u8; 32]>::from(param.conduit_key.unwrap()),
        }
    }
    pub fn sign_order(wallet: LocalWallet, order_parameters: OrderParameters, counter: U256, account_address: Option<Address>) -> Signature {
        let order_components = OrderComponents {
            offerer: order_parameters.offerer,
            zone: order_parameters.zone,
            offer: order_parameters.offer,
            consideration: order_parameters.consideration,
            order_type: order_parameters.order_type,
            start_time: order_parameters.start_time,
            end_time: order_parameters.end_time,
            zone_hash: order_parameters.zone_hash,
            salt: order_parameters.salt,
            conduit_key: order_parameters.conduit_key,
            counter,
        };
        wallet.sign_message(order_components)
    }

    // View/Read functions
    pub fn cancel(
        &self,
        orders: Vec<OrderComponents>,
        _account_address: Option<String>,
        _domain: Option<String>,
    ) -> ContractCall<M, bool> {
        let seaport = self.contract();
        seaport.cancel(orders)
    }
    pub fn validate(
        &self,
        orders: Vec<Order>,
        _account_address: Option<String>,
        _domain: Option<String>,
    ) -> ContractCall<M, bool> {
        let seaport = self.contract();
        seaport.validate(orders)
    }
    pub fn get_order_status(
        &self,
        order_hash: [u8; 32],
    ) -> ContractCall<M, (bool, bool, U256, U256)> {
        let seaport = self.contract();
        seaport.get_order_status(order_hash)
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

#[cfg(test)]
mod tests  {
    use super::*;
    use crate::constants::CROSS_CHAIN_SEAPORT_ADDRESS;
    use crate::types::SeaportConfig;
    use ethers::prelude::*;
    use std::str::FromStr;

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
