use crate::constants;
use crate::types::Overrides;
use crate::types::ProviderOrSigner;
use crate::types::SeaportConfig;
use ethers_providers::{Http, Provider};
use std::collections::HashMap;

// #[derive(Clone, Debug)]
struct Seaport<P> {
    providerOrSigner: ProviderOrSigner<P>,
    seaportConfig: Option<SeaportConfig>,
}

impl<P> Seaport<P> {
    // Options are used alot in this function because I wanted to create a situation where the function could be calle without a cfg being provided.

    pub fn new(providerOrSigner: ProviderOrSigner<P>, cfg: SeaportConfig) -> Self {

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
}

// I made this because I need to create a default config
// I found a pattern like in Opensea-rs and thought "Yeah, I could use that"
impl Default for SeaportConfig {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(
            Self::get_opensea_conduit_key(),
            *constants::OPENSEA_CONDUIT_ADDRESS,
        );

        Self {
            // Most to the value have Some covering it because I made them Options
            ascendingAmountFulfillmentBuffer: Some(300),
            balanceAndApprovalChecksOnOrderCreation: Some(true),
            conduitKeyToConduit: Some(map),
            overides: Some(Overrides {
                contractAddress: *constants::OPENSEA_CONDUIT_ADDRESS,
                defaultConduitkey: Self::get_opensea_conduit_key(),
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
        let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR_API_KEY")?;
        let cfg = SeaportConfig::default();
        let seaport = Seaport::new(ProviderOrSigner::Provider(provider), cfg);
        // println!("{:?}", seaport);
        // assert_eq!(seaport.ascendingAmountFulfillmentBuffer, Some(300));
    }
}
