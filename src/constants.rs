use std::collections::HashMap;
// use ethers::types::(Address, H256);
use ethers::types::{Address, H256};
use once_cell::sync::Lazy;
use std::str::FromStr;

pub const SEAPORT_CONTRACT_NAME: &str = "seaport";
pub const SEAPORT_CONTRACT_VERSION: f64 = 1.1;
pub const OPENSEA_CONDUIT_KEY: H256 = H256::from_str("0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000").unwrap();

pub enum OrderType {
    FullOpen = 0,
    // No partial fills, anyone can execute
    PartialOpen = 1,
    // Partial fills supported, anyone can execute
    FullRestricted = 2,
    // No partial fills, only offerer or zone can execute
    PartialRestricted = 3, // Partial fills supported, only offerer or zone can execute
}

pub enum ItemType {
    NATIVE = 0,
    ERC20 = 1,
    ERC721 = 2,
    ERC1155 = 3,
    Erc721WithCriteria = 4,
    Erc1155WithCriteria = 5,
}

pub enum Side {
    OFFER = 0,
    CONSIDERATION = 1,
}

pub enum BasicOrderRouteType {
    EthToErc721,
    EthToErc1155,
    Erc20ToErc721,
    Erc20ToErc1155,
    Erc721ToErc20,
    Erc1155ToErc20,
}

pub const ONE_HUNDRED_PERCENT_BP: u64 = 10000;
pub const NO_CONDUIT: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

pub const KNOWN_CONDUIT_KEYS_TO_CONDUIT: HashMap<H256,Lazy<Address>> =
    HashMap::from([(OPENSEA_CONDUIT_KEY, OPENSEA_CONDUIT_ADDRESS)]);

pub static OPENSEA_CONDUIT_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x1e0049783f008a0085193e00003d00cd54003c71"
        .parse()
        .unwrap()
});

pub static CROSS_CHAIN_SEAPORT_ADDRESS: Lazy<Address> = Lazy::new(|| {
    "0x00000000006c3852cbef3e08e8df289169ede581"
        .parse()
        .unwrap()
});

//
// let solar_distance = HashMap::from([
// ("Mercury", 0.4),
// ("Venus", 0.7),
// ("Earth", 1.0),
// ("Mars", 1.5),
// ]);
