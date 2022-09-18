// use crate::constants;
// use ethers::types::Address;
// use ethers_providers::{Http, Middleware, Provider};
// use serde::{Deserialize, Serialize};
// use spez;
// use std::convert::TryFrom;
use crate::types::SeaportConfig;
use crate::constants::KNOWN_CONDUIT_KEYS_TO_CONDUIT;
use crate::constants::OPENSEA_CONDUIT_ADDRESS;
use crate::constants::OPENSEA_CONDUIT_KEY;
use crate::types::Overrides;

// for types, add when you're done
// use crate::types::{}
// #[derive(Clone, Debug)]
struct Seaport<T> {
    // Making this a generic type because I dont know to set as its type
    providerOrSigner: T,
    seaportConfig: Option<SeaportConfig>,
}

//use spez, gn I'm tired
impl<T> Seaport<T> {
    pub fn new(providerOrSigner: T, cfg: SeaportConfig) -> Self {
        match cfg.ascendingAmountFulfillmentBuffer {
            Some(_x) => Self {
                providerOrSigner,
                seaportConfig: Some(cfg),
            },
            None => Self {
                providerOrSigner,
                seaportConfig: Some(SeaportConfig::default()),
            },
        }
    }
}

impl Default for SeaportConfig {
    fn default() -> Self {
        Self {
            ascendingAmountFulfillmentBuffer: Some(300),
            balanceAndApprovalChecksOnOrderCreation: Some(true),
            conduitKeyToConduit: Some(KNOWN_CONDUIT_KEYS_TO_CONDUIT),
            overides: Some(Overrides {
                contractAddress: OPENSEA_CONDUIT_ADDRESS,
                defaultConduitkey: OPENSEA_CONDUIT_KEY,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn can_get_order() {
        // let api = OpenSeaApi::new(OpenSeaApiConfig::default());
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
