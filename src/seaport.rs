use crate::constants;
use crate::types::Overrides;
use crate::types::ProviderOrSigner;
use crate::types::SeaportConfig;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Seaport {
    provider_or_signer: ProviderOrSigner,
    seaport_config: Option<SeaportConfig>,
}

impl Seaport {
    // Options are used alot in this function because I wanted to create a situation where the function could be calle without a cfg being provided.

    pub fn new(provider_or_signer: ProviderOrSigner, cfg: SeaportConfig) -> Self {
        // The idea here is to be able to tell if a config was passed
        // This line was made with the assumption that If cfg.ascending_amount_fulfillment_buffer is present then a config was provided
        // cfg.ascending_amount_fulfillment_buffer is not special to this theory, any other field could have been used
        match cfg.ascending_amount_fulfillment_buffer {
            Some(_x) => {
                return Self {
                    provider_or_signer,
                    seaport_config: Some(cfg),
                };
            }

            // This is a case when a cfg is not provided
            // I use SeaportConfig::default()-line 51 has a way to provide a default a base config
            None => {
                return Self {
                    provider_or_signer,
                    seaport_config: Some(SeaportConfig::default()),
                };
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SeaportConfig;
    use ethers::prelude::*;

    #[tokio::test]
    async fn test_default() {
        let mut map = HashMap::new();
        map.insert(
            SeaportConfig::get_opensea_conduit_key(),
            *constants::OPENSEA_CONDUIT_ADDRESS,
        );

        // let provider = Provider::<Http>::try_from("https://rinkeby.infura.io/v3/856ed0b5c16548ba845075347a2e431f")?;
        // let seaport = Seaport::new(ProviderOrSigner::Provider(provider), cfg);
        // println!("{:?}", cfg);

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
    async fn test_seaport() {
        let mut map = HashMap::new();
        map.insert(
            SeaportConfig::get_opensea_conduit_key(),
            *constants::OPENSEA_CONDUIT_ADDRESS,
        );

        // Start the stack
        let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();

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
