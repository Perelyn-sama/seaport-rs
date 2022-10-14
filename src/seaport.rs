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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::SeaportConfig;
    use ethers::prelude::*;

    #[tokio::test]
    async fn test_seaportConfig_default() {
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
