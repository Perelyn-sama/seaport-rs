use ethers::types::Address;
use once_cell::sync::Lazy;

pub const SEAPORT_CONTRACT_NAME: &str = "seaport";
pub const SEAPORT_CONTRACT_VERSION: f64 = 1.1;

#[derive(Debug)]
pub enum OrderType {
    FullOpen = 0,
    // No partial fills, anyone can execute
    PartialOpen = 1,
    // Partial fills supported, anyone can execute
    FullRestricted = 2,
    // No partial fills, only offerer or zone can execute
    PartialRestricted = 3, // Partial fills supported, only offerer or zone can execute
}

#[derive(Debug)]
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
