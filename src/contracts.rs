// use ethers::contract::abigen;
//
// abigen!(
//     Seaport,
//     r#"[
//         function cancel(OrderComponents[] calldata orders) external returns (bool cancelled)
//         function incrementCounter() external returns (uint256 newCounter)
//         function validate(Order[] calldata orders) external returns (bool validated)
//         function getOrderHash(OrderComponents calldata order) external view returns (bytes32 orderHash)
//         function getCounter(address offerer) external view returns (uint256 counter)
//         function matchOrders( Order[] calldata orders, Fulfillment[] calldata fulfillments) external payable returns (Execution[] memory executions)"
//     ]"#,
//     event_derives(serde::Deserialize, serde::Serialize)
// );
