use ethers::types::Address;
use serde::{Deserialize, Serialize};

// for types, add when you're done
// use crate::types::{}
#[derive(Clone, Debuh)]
struct Seaport {
    // Making this a generic type because I dont know to set as its type
    providerOrSigner: <T>,
    seaportConfig: Option<SeaportConfig>
}
