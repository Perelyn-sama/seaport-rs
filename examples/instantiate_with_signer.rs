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

    let signer = LocalWallet::new(&mut rand::thread_rng());

    let cfg = SeaportConfig::default();
    //    let cfg2 = SeaportConfig::default();

    let seaport: Seaport = Seaport::new(ProviderOrSigner::Signer(signer), cfg);

    let output = seaport;

    println!("{:?}", output);

    Ok(())
}
