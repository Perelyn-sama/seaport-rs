use crate::constants::CROSS_CHAIN_SEAPORT_ADDRESS;
use ethers::prelude::*;
use ethers::types::Address;
use eyre::Result;
use seaport::bindings::seaport::{Order, OrderComponents};
use seaport::constants;
use seaport::seaport::Seaport;
use seaport::types::SeaportConfig;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let mut map = HashMap::new();
    map.insert(
        SeaportConfig::get_opensea_conduit_key(),
        *constants::OPENSEA_CONDUIT_ADDRESS,
    );

    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .unwrap();

    let signer = LocalWallet::new(&mut rand::thread_rng());
    let signer_middleware = SignerMiddleware::new(provider, signer);

    let client = Arc::new(signer_middleware);

    let seaport = Seaport::new(
        client,
        Address::from_str(CROSS_CHAIN_SEAPORT_ADDRESS).unwrap(),
    )
    .build();

    // get_counter
    let offerer = Address::from_str("0x00000000006c3852cbEf3e08E8dF289169EdE581").unwrap();
    let counter = seaport.get_counter(offerer).await?;
    dbg!(counter);

    // cancel
    let create_order_input = vec![OrderComponents::default()];
    let _ = dbg!(seaport.cancel(create_order_input, None, None));

    // validate
    let order_input = vec![Order::default()];
    let _ = dbg!(seaport.validate(order_input, None, None));

    // get_order_status
    //  FIXME get order hash
    // let order_hash;
    // dbg!(seaport.get_order_status());

    Ok(())
}
