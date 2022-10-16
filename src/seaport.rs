use crate::constants;
use crate::constants::ItemType;
use crate::types::{
    BasicErc721Item, ConsiderationInputItem, CreateInputItem, CreateOrderInput, CurrencyItem,
    Erc721Item, Overrides, ProviderOrSigner, SeaportConfig,
};
use ethers::types::U256;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Seaport {
    provider_or_signer: ProviderOrSigner,
    seaport_config: Option<SeaportConfig>,
}

impl Seaport {
    pub fn new(provider_or_signer: ProviderOrSigner, cfg: SeaportConfig) -> Self {
        match cfg.ascending_amount_fulfillment_buffer {
            Some(_x) => {
                return Self {
                    provider_or_signer,
                    seaport_config: Some(cfg),
                };
            }

            None => {
                return Self {
                    provider_or_signer,
                    seaport_config: Some(SeaportConfig::default()),
                };
            }
        }
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
            conduit_key: Some(*constants::OPENSEA_CONDUIT_ADDRESS),
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
    use crate::types::SeaportConfig;
    use ethers::prelude::*;

    #[tokio::test]
    async fn test_seaport_config_default() {
        let mut map = HashMap::new();
        map.insert(
            SeaportConfig::get_opensea_conduit_key(),
            *constants::OPENSEA_CONDUIT_ADDRESS,
        );

        let cfg = SeaportConfig::default();
        let output = SeaportConfig::from(cfg);

        assert_eq!(output.ascending_amount_fulfillment_buffer, Some(300));
        assert_eq!(output.ascending_amount_fulfillment_buffer.unwrap(), 300);
        assert_eq!(
            output
                .balance_and_approval_checks_on_order_creation
                .unwrap(),
            true
        );
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

        // Start the stack
        //        let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();

        // Sign transactions with a private key
        let signer = LocalWallet::new(&mut rand::thread_rng());
        // let address = signer.address();
        // let provider = SignerMiddleware::new(provider, signer);

        let cfg = SeaportConfig::default();
        let cfg2 = SeaportConfig::default();

        let seaport: Seaport = Seaport::new(ProviderOrSigner::Signer(signer), cfg);

        let output = Seaport::from(seaport);

        assert_eq!(output.seaport_config.unwrap(), cfg2);
    }
}
