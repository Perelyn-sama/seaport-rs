// use ethers::prelude::*;
// use eyre::Result;
// use std::sync::Arc;

// // Generate the type-safe contract bindings by providing the ABI
// // definition in human readable format
// abigen!(
//     IUniswapV2Pair,
//     r#"[
//         function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)
//     ]"#,
// );

// #[tokio::main]
// pub async fn main() -> Result<()> {
//     let client = Provider::<Http>::try_from(
//         "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27",
//     )?;
//     let client = Arc::new(client);

//     // ETH/USDT pair on Uniswap V2
//     let address = "0x00000000006c3852cbEf3e08E8dF289169EdE581".parse::<Address>()?;
//     let pair = IUniswapV2Pair::new(address, Arc::clone(&client));

//     // getReserves -> get_reserves
//     let (reserve0, reserve1, _timestamp) = pair.get_reserves().call().await?;
//     println!("Reserves (ETH, USDT): ({}, {})", reserve0, reserve1);

//     let mid_price = f64::powi(10.0, 18 - 6) * reserve1 as f64 / reserve0 as f64;
//     println!("ETH/USDT price: {:.2}", mid_price);
//     Ok(())
// }
