use ethers::prelude::*;
use eyre::Result;
use seaport::constants;
use seaport::seaport::Seaport;
use seaport::types::{ProviderOrSigner, SeaportConfig};
use std::collections::HashMap;

fn main() -> Result<()> {
    let mut map = HashMap::new();
    map.insert(
        SeaportConfig::get_opensea_conduit_key(),
        *constants::OPENSEA_CONDUIT_ADDRESS,
    );

    let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();

    let cfg = SeaportConfig::default();

    let seaport: Seaport = Seaport::new(ProviderOrSigner::Provider(provider), cfg);

    let output = Seaport::from(seaport);

    println!("{:?}", output);

    Ok(())
}
