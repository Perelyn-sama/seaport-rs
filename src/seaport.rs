use crate::constants::OPENSEA_CONDUIT_ADDRESS;
use ethers::types::{Address, H256};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::str::FromStr;
use crate::types::ProviderOrSigner;
use crate::types::SeaportConfig;
use crate::types::Overrides;

// This is where most of the work in the project will be done
// Most of the functions I want to implement will be here but first I have to get the new function to work :)

// I have no idea why people use this patten of structs and impls, I don't know what the pattern is called or its advantages, I saw it off Opensea-rs and some other code bases


// #[derive(Clone, Debug)]
struct Seaport {
    providerOrSigner: ProviderOrSigner,
    seaportConfig: Option<SeaportConfig>,
}

//use spez, gn I'm tired
impl Seaport {
    // Options are used alot in this function because I wanted to create a situation where the function could be calle without a cfg being provided.
    pub fn new(providerOrSigner: ProviderOrSigner, cfg: SeaportConfig) -> Self {
        // The idea here is to be able to tell if a config was passed
        // This line was made with the assumption that If cfg.ascendingAmountFulfillmentBuffer is present then a config was provided
        // cfg.ascendingAmountFulfillmentBuffer is not special to this theory, any other field could have been used
        match cfg.ascendingAmountFulfillmentBuffer {
            Some(_x) => Self {
                providerOrSigner,
                seaportConfig: Some(cfg),
            },
            // This is a case when a cfg is not provided
            // I use SeaportConfig::default()-line 51 has a way to provide a default a base config
            None => Self {
                providerOrSigner,
                seaportConfig: Some(SeaportConfig::default()),
            },
        }
    }

    // I faced issues setting the conduit key to a constanst, so I'm using a function for it instead - Got the idea from asnared :)
    // I might move this to SeaportConfig
    pub fn get_opensea_conduit_key() -> H256 {
        H256::from_str("0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000").unwrap()
    }

    // Added this function because of the same issue as the one above
    pub fn get_known_conduit_keys_to_conduit() -> HashMap<H256, Lazy<Address>> {
        HashMap::from([Self::get_opensea_conduit_key(), OPENSEA_CONDUIT_ADDRESS])
    }
}

// I made this because I need to create a default config
// I found a pattern like in Opensea-rs and thought "Yeah, I could use that"
impl Default for SeaportConfig {
    fn default() -> Self {
        Self {
            // Most to the value have Some covering it because I made them Options
            ascendingAmountFulfillmentBuffer: Some(300),
            balanceAndApprovalChecksOnOrderCreation: Some(true),
            conduitKeyToConduit: Some(Seaport::get_known_conduit_keys_to_conduit()),
            overides: Some(Overrides {
                contractAddress: OPENSEA_CONDUIT_ADDRESS,
                defaultConduitkey: Seaport::get_opensea_conduit_key(),
            }),
        }
    }
}

// Here's the part where I test If my code actually works - that's if I can escape the rust compiler first
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    // I don't think I need it to be async right now
    // Might remove this when I resume testing
    async fn can_get_order() {
        let seaport = Seaport::new(SeaportConfig::default());
        println!("{:?}", seaport);

        // let req = OrderRequest {
        //     side: 1,
        //     token_id: 2292.to_string(),
        //     contract_address: "0x7d256d82b32d8003d1ca1a1526ed211e6e0da9e2"
        //         .parse()
        //         .unwrap(),
        //     limit: 99,
        // };
        // let addr = req.contract_address;
        // let order = api.get_order(req).await.unwrap();
        // let order = MinimalOrder::from(order);
        // assert_eq!(order.target, addr);
        // assert_eq!(order.maker_relayer_fee, 600.into());
        // assert_eq!(order.taker_relayer_fee, 0.into());
        // assert_eq!(order.maker_protocol_fee, 0.into());
        // assert_eq!(order.taker_protocol_fee, 0.into());
    }
}
