use ethers::prelude::*;
use eyre::Result;
use seaport::prelude::*;
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

    let account_address = Address::from_str("0x00000000006c3852cbEf3e08E8dF289169EdE581").unwrap();
    let create_order_cfg = CreateOrderInput::default();

    dbg!(seaport.create_order(create_order_cfg, Some(account_address)));

    Ok(())
}
