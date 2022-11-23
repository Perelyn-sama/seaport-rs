pub use self::seaport::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod seaport {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "seaport was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduitController\",\"type\":\"address\"}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\"},{\"inputs\":[],\"name\":\"BadContractSignature\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"BadFraction\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"BadReturnValueFromERC20OnTransfer\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\"}],\"name\":\"BadSignatureV\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"ConsiderationCriteriaResolverOutOfRange\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"considerationIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"shortfallAmount\",\"type\":\"uint256\"}],\"name\":\"ConsiderationNotMet\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"CriteriaNotEnabledForItem\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256[]\",\"name\":\"identifiers\",\"type\":\"uint256[]\"},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\"}],\"name\":\"ERC1155BatchTransferGenericFailure\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"EtherTransferGenericFailure\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InexactFraction\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InsufficientEtherSupplied\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"Invalid1155BatchTransferEncoding\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidBasicOrderParameterEncoding\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\"}],\"name\":\"InvalidCallToConduit\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidCanceller\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\"}],\"name\":\"InvalidConduit\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidERC721TransferAmount\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidFulfillmentComponentData\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\"}],\"name\":\"InvalidMsgValue\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidNativeOfferItem\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidProof\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"}],\"name\":\"InvalidRestrictedOrder\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidSignature\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidSigner\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"InvalidTime\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MismatchedFulfillmentOfferAndConsiderationComponents\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\"}],\"name\":\"MissingFulfillmentComponentOnAggregation\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MissingItemAmount\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"MissingOriginalConsiderationItems\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\"}],\"name\":\"NoContract\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NoReentrantCalls\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"NoSpecifiedOrdersAvailable\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"OfferAndConsiderationRequiredOnFulfillment\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"OfferCriteriaResolverOutOfRange\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"}],\"name\":\"OrderAlreadyFilled\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"OrderCriteriaResolverOutOfRange\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"}],\"name\":\"OrderIsCancelled\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"}],\"name\":\"OrderPartiallyFilled\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"PartialFillsNotEnabledForOrder\",\"type\":\"error\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"name\":\"TokenTransferGenericFailure\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UnresolvedConsiderationCriteria\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UnresolvedOfferCriteria\",\"type\":\"error\"},{\"inputs\":[],\"name\":\"UnusedItemParameters\",\"type\":\"error\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"uint256\",\"name\":\"newCounter\",\"type\":\"uint256\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"}],\"name\":\"CounterIncremented\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"}],\"name\":\"OrderCancelled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"indexed\":false,\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"}],\"indexed\":false,\"internalType\":\"struct SpentItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"indexed\":false,\"internalType\":\"struct ReceivedItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"}],\"name\":\"OrderFulfilled\",\"type\":\"event\"},{\"anonymous\":false,\"inputs\":[{\"indexed\":false,\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"indexed\":true,\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"}],\"name\":\"OrderValidated\",\"type\":\"event\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"counter\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderComponents[]\",\"name\":\"orders\",\"type\":\"tuple[]\"}],\"name\":\"cancel\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"cancelled\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"},{\"internalType\":\"uint120\",\"name\":\"numerator\",\"type\":\"uint120\"},{\"internalType\":\"uint120\",\"name\":\"denominator\",\"type\":\"uint120\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"extraData\",\"type\":\"bytes\"}],\"internalType\":\"struct AdvancedOrder\",\"name\":\"advancedOrder\",\"type\":\"tuple\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"bytes32[]\",\"name\":\"criteriaProof\",\"type\":\"bytes32[]\"}],\"internalType\":\"struct CriteriaResolver[]\",\"name\":\"criteriaResolvers\",\"type\":\"tuple[]\"},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"}],\"name\":\"fulfillAdvancedOrder\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"fulfilled\",\"type\":\"bool\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"},{\"internalType\":\"uint120\",\"name\":\"numerator\",\"type\":\"uint120\"},{\"internalType\":\"uint120\",\"name\":\"denominator\",\"type\":\"uint120\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"extraData\",\"type\":\"bytes\"}],\"internalType\":\"struct AdvancedOrder[]\",\"name\":\"advancedOrders\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"bytes32[]\",\"name\":\"criteriaProof\",\"type\":\"bytes32[]\"}],\"internalType\":\"struct CriteriaResolver[]\",\"name\":\"criteriaResolvers\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"offerFulfillments\",\"type\":\"tuple[][]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"considerationFulfillments\",\"type\":\"tuple[][]\"},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"maximumFulfilled\",\"type\":\"uint256\"}],\"name\":\"fulfillAvailableAdvancedOrders\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"availableOrders\",\"type\":\"bool[]\"},{\"components\":[{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"}],\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct Order[]\",\"name\":\"orders\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"offerFulfillments\",\"type\":\"tuple[][]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"considerationFulfillments\",\"type\":\"tuple[][]\"},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"maximumFulfilled\",\"type\":\"uint256\"}],\"name\":\"fulfillAvailableOrders\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"availableOrders\",\"type\":\"bool[]\"},{\"components\":[{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"}],\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"considerationToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"considerationIdentifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"considerationAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"offerToken\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"offerIdentifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"offerAmount\",\"type\":\"uint256\"},{\"internalType\":\"enum BasicOrderType\",\"name\":\"basicOrderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"offererConduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalAdditionalRecipients\",\"type\":\"uint256\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct AdditionalRecipient[]\",\"name\":\"additionalRecipients\",\"type\":\"tuple[]\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct BasicOrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"}],\"name\":\"fulfillBasicOrder\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"fulfilled\",\"type\":\"bool\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct Order\",\"name\":\"order\",\"type\":\"tuple\"},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\"}],\"name\":\"fulfillOrder\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"fulfilled\",\"type\":\"bool\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"}],\"name\":\"getCounter\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"counter\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"counter\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderComponents\",\"name\":\"order\",\"type\":\"tuple\"}],\"name\":\"getOrderHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\"}],\"name\":\"getOrderStatus\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isValidated\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"isCancelled\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"totalFilled\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"totalSize\",\"type\":\"uint256\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"incrementCounter\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"newCounter\",\"type\":\"uint256\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"information\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\"},{\"internalType\":\"bytes32\",\"name\":\"domainSeparator\",\"type\":\"bytes32\"},{\"internalType\":\"address\",\"name\":\"conduitController\",\"type\":\"address\"}],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"},{\"internalType\":\"uint120\",\"name\":\"numerator\",\"type\":\"uint120\"},{\"internalType\":\"uint120\",\"name\":\"denominator\",\"type\":\"uint120\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"},{\"internalType\":\"bytes\",\"name\":\"extraData\",\"type\":\"bytes\"}],\"internalType\":\"struct AdvancedOrder[]\",\"name\":\"advancedOrders\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"bytes32[]\",\"name\":\"criteriaProof\",\"type\":\"bytes32[]\"}],\"internalType\":\"struct CriteriaResolver[]\",\"name\":\"criteriaResolvers\",\"type\":\"tuple[]\"},{\"components\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"offerComponents\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"considerationComponents\",\"type\":\"tuple[]\"}],\"internalType\":\"struct Fulfillment[]\",\"name\":\"fulfillments\",\"type\":\"tuple[]\"}],\"name\":\"matchAdvancedOrders\",\"outputs\":[{\"components\":[{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"}],\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct Order[]\",\"name\":\"orders\",\"type\":\"tuple[]\"},{\"components\":[{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"offerComponents\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\"}],\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"considerationComponents\",\"type\":\"tuple[]\"}],\"internalType\":\"struct Fulfillment[]\",\"name\":\"fulfillments\",\"type\":\"tuple[]\"}],\"name\":\"matchOrders\",\"outputs\":[{\"components\":[{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\"},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"}],\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\"}],\"stateMutability\":\"payable\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"contractName\",\"type\":\"string\"}],\"stateMutability\":\"pure\",\"type\":\"function\"},{\"inputs\":[{\"components\":[{\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"}],\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\"},{\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\"},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"naßme\":\"startAmount\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\"},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\"}],\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\"},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\"},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\"},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\"},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\"}],\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\"},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\"}],\"internalType\":\"struct Order[]\",\"name\":\"orders\",\"type\":\"tuple[]\"}],\"name\":\"validate\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"validated\",\"type\":\"bool\"}],\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static SEAPORT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct seaport<M>(ethers::contract::Contract<M>);
    impl<M> Clone for seaport<M> {
        fn clone(&self) -> Self {
            seaport(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for seaport<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for seaport<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(seaport))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> seaport<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SEAPORT_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `cancel` (0xfd9f1e10) function"]
        pub fn cancel(
            &self,
            orders: ::std::vec::Vec<OrderComponents>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([253, 159, 30, 16], orders)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillAdvancedOrder` (0xe7acab24) function"]
        pub fn fulfill_advanced_order(
            &self,
            advanced_order: AdvancedOrder,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            fulfiller_conduit_key: [u8; 32],
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [231, 172, 171, 36],
                    (
                        advanced_order,
                        criteria_resolvers,
                        fulfiller_conduit_key,
                        recipient,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillAvailableAdvancedOrders` (0x87201b41) function"]
        pub fn fulfill_available_advanced_orders(
            &self,
            advanced_orders: ::std::vec::Vec<AdvancedOrder>,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            fulfiller_conduit_key: [u8; 32],
            recipient: ethers::core::types::Address,
            maximum_fulfilled: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<bool>, ::std::vec::Vec<Execution>),
        > {
            self.0
                .method_hash(
                    [135, 32, 27, 65],
                    (
                        advanced_orders,
                        criteria_resolvers,
                        offer_fulfillments,
                        consideration_fulfillments,
                        fulfiller_conduit_key,
                        recipient,
                        maximum_fulfilled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillAvailableOrders` (0xed98a574) function"]
        pub fn fulfill_available_orders(
            &self,
            orders: ::std::vec::Vec<Order>,
            offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            fulfiller_conduit_key: [u8; 32],
            maximum_fulfilled: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<bool>, ::std::vec::Vec<Execution>),
        > {
            self.0
                .method_hash(
                    [237, 152, 165, 116],
                    (
                        orders,
                        offer_fulfillments,
                        consideration_fulfillments,
                        fulfiller_conduit_key,
                        maximum_fulfilled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillBasicOrder` (0xfb0f3ee1) function"]
        pub fn fulfill_basic_order(
            &self,
            parameters: BasicOrderParameters,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([251, 15, 62, 225], (parameters,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillOrder` (0xb3a34c4c) function"]
        pub fn fulfill_order(
            &self,
            order: Order,
            fulfiller_conduit_key: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 163, 76, 76], (order, fulfiller_conduit_key))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCounter` (0xf07ec373) function"]
        pub fn get_counter(
            &self,
            offerer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([240, 126, 195, 115], offerer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOrderHash` (0x79df72bd) function"]
        pub fn get_order_hash(
            &self,
            order: OrderComponents,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 223, 114, 189], (order,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOrderStatus` (0x46423aa7) function"]
        pub fn get_order_status(
            &self,
            order_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                bool,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([70, 66, 58, 167], order_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incrementCounter` (0x5b34b966) function"]
        pub fn increment_counter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([91, 52, 185, 102], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `information` (0xf47b7740) function"]
        pub fn information(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (String, [u8; 32], ethers::core::types::Address),
        > {
            self.0
                .method_hash([244, 123, 119, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchAdvancedOrders` (0x55944a42) function"]
        pub fn match_advanced_orders(
            &self,
            advanced_orders: ::std::vec::Vec<AdvancedOrder>,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            fulfillments: ::std::vec::Vec<Fulfillment>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Execution>> {
            self.0
                .method_hash(
                    [85, 148, 74, 66],
                    (advanced_orders, criteria_resolvers, fulfillments),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchOrders` (0xa8174404) function"]
        pub fn match_orders(
            &self,
            orders: ::std::vec::Vec<Order>,
            fulfillments: ::std::vec::Vec<Fulfillment>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Execution>> {
            self.0
                .method_hash([168, 23, 68, 4], (orders, fulfillments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validate` (0x88147732) function"]
        pub fn validate(
            &self,
            orders: ::std::vec::Vec<Order>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([136, 20, 119, 50], orders)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CounterIncremented` event"]
        pub fn counter_incremented_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CounterIncrementedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderCancelled` event"]
        pub fn order_cancelled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderCancelledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderFulfilled` event"]
        pub fn order_fulfilled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderFulfilledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderValidated` event"]
        pub fn order_validated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderValidatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, seaportEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for seaport<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[doc = "Custom Error type `BadContractSignature` with signature `BadContractSignature()` and selector `[79, 127, 184, 13]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "BadContractSignature", abi = "BadContractSignature()")]
    pub struct BadContractSignature;
    #[doc = "Custom Error type `BadFraction` with signature `BadFraction()` and selector `[90, 5, 43, 50]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "BadFraction", abi = "BadFraction()")]
    pub struct BadFraction;
    #[doc = "Custom Error type `BadReturnValueFromERC20OnTransfer` with signature `BadReturnValueFromERC20OnTransfer(address,address,address,uint256)` and selector `[152, 137, 25, 35]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "BadReturnValueFromERC20OnTransfer",
        abi = "BadReturnValueFromERC20OnTransfer(address,address,address,uint256)"
    )]
    pub struct BadReturnValueFromERC20OnTransfer {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `BadSignatureV` with signature `BadSignatureV(uint8)` and selector `[31, 0, 61, 10]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "BadSignatureV", abi = "BadSignatureV(uint8)")]
    pub struct BadSignatureV {
        pub v: u8,
    }
    #[doc = "Custom Error type `ConsiderationCriteriaResolverOutOfRange` with signature `ConsiderationCriteriaResolverOutOfRange()` and selector `[96, 136, 215, 222]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "ConsiderationCriteriaResolverOutOfRange",
        abi = "ConsiderationCriteriaResolverOutOfRange()"
    )]
    pub struct ConsiderationCriteriaResolverOutOfRange;
    #[doc = "Custom Error type `ConsiderationNotMet` with signature `ConsiderationNotMet(uint256,uint256,uint256)` and selector `[165, 245, 66, 8]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "ConsiderationNotMet",
        abi = "ConsiderationNotMet(uint256,uint256,uint256)"
    )]
    pub struct ConsiderationNotMet {
        pub order_index: ethers::core::types::U256,
        pub consideration_index: ethers::core::types::U256,
        pub shortfall_amount: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `CriteriaNotEnabledForItem` with signature `CriteriaNotEnabledForItem()` and selector `[148, 235, 106, 246]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "CriteriaNotEnabledForItem",
        abi = "CriteriaNotEnabledForItem()"
    )]
    pub struct CriteriaNotEnabledForItem;
    #[doc = "Custom Error type `ERC1155BatchTransferGenericFailure` with signature `ERC1155BatchTransferGenericFailure(address,address,address,uint256[],uint256[])` and selector `[175, 196, 69, 226]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "ERC1155BatchTransferGenericFailure",
        abi = "ERC1155BatchTransferGenericFailure(address,address,address,uint256[],uint256[])"
    )]
    pub struct ERC1155BatchTransferGenericFailure {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub identifiers: ::std::vec::Vec<ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<ethers::core::types::U256>,
    }
    #[doc = "Custom Error type `EtherTransferGenericFailure` with signature `EtherTransferGenericFailure(address,uint256)` and selector `[71, 12, 124, 29]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "EtherTransferGenericFailure",
        abi = "EtherTransferGenericFailure(address,uint256)"
    )]
    pub struct EtherTransferGenericFailure {
        pub account: ethers::core::types::Address,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `InexactFraction` with signature `InexactFraction()` and selector `[198, 60, 240, 137]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InexactFraction", abi = "InexactFraction()")]
    pub struct InexactFraction;
    #[doc = "Custom Error type `InsufficientEtherSupplied` with signature `InsufficientEtherSupplied()` and selector `[26, 120, 59, 141]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "InsufficientEtherSupplied",
        abi = "InsufficientEtherSupplied()"
    )]
    pub struct InsufficientEtherSupplied;
    #[doc = "Custom Error type `Invalid1155BatchTransferEncoding` with signature `Invalid1155BatchTransferEncoding()` and selector `[235, 162, 8, 76]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "Invalid1155BatchTransferEncoding",
        abi = "Invalid1155BatchTransferEncoding()"
    )]
    pub struct Invalid1155BatchTransferEncoding;
    #[doc = "Custom Error type `InvalidBasicOrderParameterEncoding` with signature `InvalidBasicOrderParameterEncoding()` and selector `[57, 243, 227, 253]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "InvalidBasicOrderParameterEncoding",
        abi = "InvalidBasicOrderParameterEncoding()"
    )]
    pub struct InvalidBasicOrderParameterEncoding;
    #[doc = "Custom Error type `InvalidCallToConduit` with signature `InvalidCallToConduit(address)` and selector `[209, 61, 83, 212]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidCallToConduit", abi = "InvalidCallToConduit(address)")]
    pub struct InvalidCallToConduit {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Custom Error type `InvalidCanceller` with signature `InvalidCanceller()` and selector `[128, 236, 115, 116]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidCanceller", abi = "InvalidCanceller()")]
    pub struct InvalidCanceller;
    #[doc = "Custom Error type `InvalidConduit` with signature `InvalidConduit(bytes32,address)` and selector `[28, 249, 155, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidConduit", abi = "InvalidConduit(bytes32,address)")]
    pub struct InvalidConduit {
        pub conduit_key: [u8; 32],
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Custom Error type `InvalidERC721TransferAmount` with signature `InvalidERC721TransferAmount()` and selector `[239, 204, 0, 177]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "InvalidERC721TransferAmount",
        abi = "InvalidERC721TransferAmount()"
    )]
    pub struct InvalidERC721TransferAmount;
    #[doc = "Custom Error type `InvalidFulfillmentComponentData` with signature `InvalidFulfillmentComponentData()` and selector `[127, 218, 114, 121]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "InvalidFulfillmentComponentData",
        abi = "InvalidFulfillmentComponentData()"
    )]
    pub struct InvalidFulfillmentComponentData;
    #[doc = "Custom Error type `InvalidMsgValue` with signature `InvalidMsgValue(uint256)` and selector `[166, 27, 233, 240]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidMsgValue", abi = "InvalidMsgValue(uint256)")]
    pub struct InvalidMsgValue {
        pub value: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `InvalidNativeOfferItem` with signature `InvalidNativeOfferItem()` and selector `[18, 211, 245, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidNativeOfferItem", abi = "InvalidNativeOfferItem()")]
    pub struct InvalidNativeOfferItem;
    #[doc = "Custom Error type `InvalidProof` with signature `InvalidProof()` and selector `[9, 189, 227, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidProof", abi = "InvalidProof()")]
    pub struct InvalidProof;
    #[doc = "Custom Error type `InvalidRestrictedOrder` with signature `InvalidRestrictedOrder(bytes32)` and selector `[251, 80, 20, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "InvalidRestrictedOrder",
        abi = "InvalidRestrictedOrder(bytes32)"
    )]
    pub struct InvalidRestrictedOrder {
        pub order_hash: [u8; 32],
    }
    #[doc = "Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `[139, 170, 87, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    #[doc = "Custom Error type `InvalidSigner` with signature `InvalidSigner()` and selector `[129, 94, 29, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidSigner", abi = "InvalidSigner()")]
    pub struct InvalidSigner;
    #[doc = "Custom Error type `InvalidTime` with signature `InvalidTime()` and selector `[111, 126, 172, 38]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "InvalidTime", abi = "InvalidTime()")]
    pub struct InvalidTime;
    #[doc = "Custom Error type `MismatchedFulfillmentOfferAndConsiderationComponents` with signature `MismatchedFulfillmentOfferAndConsiderationComponents()` and selector `[9, 207, 180, 85]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MismatchedFulfillmentOfferAndConsiderationComponents",
        abi = "MismatchedFulfillmentOfferAndConsiderationComponents()"
    )]
    pub struct MismatchedFulfillmentOfferAndConsiderationComponents;
    #[doc = "Custom Error type `MissingFulfillmentComponentOnAggregation` with signature `MissingFulfillmentComponentOnAggregation(uint8)` and selector `[55, 92, 36, 193]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MissingFulfillmentComponentOnAggregation",
        abi = "MissingFulfillmentComponentOnAggregation(uint8)"
    )]
    pub struct MissingFulfillmentComponentOnAggregation {
        pub side: u8,
    }
    #[doc = "Custom Error type `MissingItemAmount` with signature `MissingItemAmount()` and selector `[145, 179, 229, 20]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "MissingItemAmount", abi = "MissingItemAmount()")]
    pub struct MissingItemAmount;
    #[doc = "Custom Error type `MissingOriginalConsiderationItems` with signature `MissingOriginalConsiderationItems()` and selector `[70, 106, 166, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "MissingOriginalConsiderationItems",
        abi = "MissingOriginalConsiderationItems()"
    )]
    pub struct MissingOriginalConsiderationItems;
    #[doc = "Custom Error type `NoContract` with signature `NoContract(address)` and selector `[95, 21, 214, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NoContract", abi = "NoContract(address)")]
    pub struct NoContract {
        pub account: ethers::core::types::Address,
    }
    #[doc = "Custom Error type `NoReentrantCalls` with signature `NoReentrantCalls()` and selector `[127, 168, 169, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "NoReentrantCalls", abi = "NoReentrantCalls()")]
    pub struct NoReentrantCalls;
    #[doc = "Custom Error type `NoSpecifiedOrdersAvailable` with signature `NoSpecifiedOrdersAvailable()` and selector `[213, 218, 154, 27]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "NoSpecifiedOrdersAvailable",
        abi = "NoSpecifiedOrdersAvailable()"
    )]
    pub struct NoSpecifiedOrdersAvailable;
    #[doc = "Custom Error type `OfferAndConsiderationRequiredOnFulfillment` with signature `OfferAndConsiderationRequiredOnFulfillment()` and selector `[152, 233, 219, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "OfferAndConsiderationRequiredOnFulfillment",
        abi = "OfferAndConsiderationRequiredOnFulfillment()"
    )]
    pub struct OfferAndConsiderationRequiredOnFulfillment;
    #[doc = "Custom Error type `OfferCriteriaResolverOutOfRange` with signature `OfferCriteriaResolverOutOfRange()` and selector `[191, 179, 248, 206]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "OfferCriteriaResolverOutOfRange",
        abi = "OfferCriteriaResolverOutOfRange()"
    )]
    pub struct OfferCriteriaResolverOutOfRange;
    #[doc = "Custom Error type `OrderAlreadyFilled` with signature `OrderAlreadyFilled(bytes32)` and selector `[16, 253, 163, 225]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OrderAlreadyFilled", abi = "OrderAlreadyFilled(bytes32)")]
    pub struct OrderAlreadyFilled {
        pub order_hash: [u8; 32],
    }
    #[doc = "Custom Error type `OrderCriteriaResolverOutOfRange` with signature `OrderCriteriaResolverOutOfRange()` and selector `[134, 149, 134, 196]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "OrderCriteriaResolverOutOfRange",
        abi = "OrderCriteriaResolverOutOfRange()"
    )]
    pub struct OrderCriteriaResolverOutOfRange;
    #[doc = "Custom Error type `OrderIsCancelled` with signature `OrderIsCancelled(bytes32)` and selector `[26, 81, 85, 116]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OrderIsCancelled", abi = "OrderIsCancelled(bytes32)")]
    pub struct OrderIsCancelled {
        pub order_hash: [u8; 32],
    }
    #[doc = "Custom Error type `OrderPartiallyFilled` with signature `OrderPartiallyFilled(bytes32)` and selector `[238, 158, 14, 99]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "OrderPartiallyFilled", abi = "OrderPartiallyFilled(bytes32)")]
    pub struct OrderPartiallyFilled {
        pub order_hash: [u8; 32],
    }
    #[doc = "Custom Error type `PartialFillsNotEnabledForOrder` with signature `PartialFillsNotEnabledForOrder()` and selector `[161, 27, 99, 255]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "PartialFillsNotEnabledForOrder",
        abi = "PartialFillsNotEnabledForOrder()"
    )]
    pub struct PartialFillsNotEnabledForOrder;
    #[doc = "Custom Error type `TokenTransferGenericFailure` with signature `TokenTransferGenericFailure(address,address,address,uint256,uint256)` and selector `[244, 134, 188, 135]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "TokenTransferGenericFailure",
        abi = "TokenTransferGenericFailure(address,address,address,uint256,uint256)"
    )]
    pub struct TokenTransferGenericFailure {
        pub token: ethers::core::types::Address,
        pub from: ethers::core::types::Address,
        pub to: ethers::core::types::Address,
        pub identifier: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
    #[doc = "Custom Error type `UnresolvedConsiderationCriteria` with signature `UnresolvedConsiderationCriteria()` and selector `[255, 117, 163, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(
        name = "UnresolvedConsiderationCriteria",
        abi = "UnresolvedConsiderationCriteria()"
    )]
    pub struct UnresolvedConsiderationCriteria;
    #[doc = "Custom Error type `UnresolvedOfferCriteria` with signature `UnresolvedOfferCriteria()` and selector `[166, 207, 198, 115]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "UnresolvedOfferCriteria", abi = "UnresolvedOfferCriteria()")]
    pub struct UnresolvedOfferCriteria;
    #[doc = "Custom Error type `UnusedItemParameters` with signature `UnusedItemParameters()` and selector `[106, 179, 124, 231]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthError,
        ethers :: contract :: EthDisplay,
    )]
    #[etherror(name = "UnusedItemParameters", abi = "UnusedItemParameters()")]
    pub struct UnusedItemParameters;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum seaportErrors {
        BadContractSignature(BadContractSignature),
        BadFraction(BadFraction),
        BadReturnValueFromERC20OnTransfer(BadReturnValueFromERC20OnTransfer),
        BadSignatureV(BadSignatureV),
        ConsiderationCriteriaResolverOutOfRange(ConsiderationCriteriaResolverOutOfRange),
        ConsiderationNotMet(ConsiderationNotMet),
        CriteriaNotEnabledForItem(CriteriaNotEnabledForItem),
        ERC1155BatchTransferGenericFailure(ERC1155BatchTransferGenericFailure),
        EtherTransferGenericFailure(EtherTransferGenericFailure),
        InexactFraction(InexactFraction),
        InsufficientEtherSupplied(InsufficientEtherSupplied),
        Invalid1155BatchTransferEncoding(Invalid1155BatchTransferEncoding),
        InvalidBasicOrderParameterEncoding(InvalidBasicOrderParameterEncoding),
        InvalidCallToConduit(InvalidCallToConduit),
        InvalidCanceller(InvalidCanceller),
        InvalidConduit(InvalidConduit),
        InvalidERC721TransferAmount(InvalidERC721TransferAmount),
        InvalidFulfillmentComponentData(InvalidFulfillmentComponentData),
        InvalidMsgValue(InvalidMsgValue),
        InvalidNativeOfferItem(InvalidNativeOfferItem),
        InvalidProof(InvalidProof),
        InvalidRestrictedOrder(InvalidRestrictedOrder),
        InvalidSignature(InvalidSignature),
        InvalidSigner(InvalidSigner),
        InvalidTime(InvalidTime),
        MismatchedFulfillmentOfferAndConsiderationComponents(
            MismatchedFulfillmentOfferAndConsiderationComponents,
        ),
        MissingFulfillmentComponentOnAggregation(MissingFulfillmentComponentOnAggregation),
        MissingItemAmount(MissingItemAmount),
        MissingOriginalConsiderationItems(MissingOriginalConsiderationItems),
        NoContract(NoContract),
        NoReentrantCalls(NoReentrantCalls),
        NoSpecifiedOrdersAvailable(NoSpecifiedOrdersAvailable),
        OfferAndConsiderationRequiredOnFulfillment(OfferAndConsiderationRequiredOnFulfillment),
        OfferCriteriaResolverOutOfRange(OfferCriteriaResolverOutOfRange),
        OrderAlreadyFilled(OrderAlreadyFilled),
        OrderCriteriaResolverOutOfRange(OrderCriteriaResolverOutOfRange),
        OrderIsCancelled(OrderIsCancelled),
        OrderPartiallyFilled(OrderPartiallyFilled),
        PartialFillsNotEnabledForOrder(PartialFillsNotEnabledForOrder),
        TokenTransferGenericFailure(TokenTransferGenericFailure),
        UnresolvedConsiderationCriteria(UnresolvedConsiderationCriteria),
        UnresolvedOfferCriteria(UnresolvedOfferCriteria),
        UnusedItemParameters(UnusedItemParameters),
    }
    impl ethers::core::abi::AbiDecode for seaportErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <BadContractSignature as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::BadContractSignature(decoded));
            }
            if let Ok(decoded) =
                <BadFraction as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::BadFraction(decoded));
            }
            if let Ok(decoded) =
                <BadReturnValueFromERC20OnTransfer as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::BadReturnValueFromERC20OnTransfer(decoded));
            }
            if let Ok(decoded) =
                <BadSignatureV as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::BadSignatureV(decoded));
            }
            if let Ok(decoded) =
                <ConsiderationCriteriaResolverOutOfRange as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::ConsiderationCriteriaResolverOutOfRange(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <ConsiderationNotMet as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::ConsiderationNotMet(decoded));
            }
            if let Ok(decoded) =
                <CriteriaNotEnabledForItem as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::CriteriaNotEnabledForItem(decoded));
            }
            if let Ok(decoded) =
                <ERC1155BatchTransferGenericFailure as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::ERC1155BatchTransferGenericFailure(decoded));
            }
            if let Ok(decoded) =
                <EtherTransferGenericFailure as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::EtherTransferGenericFailure(decoded));
            }
            if let Ok(decoded) =
                <InexactFraction as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InexactFraction(decoded));
            }
            if let Ok(decoded) =
                <InsufficientEtherSupplied as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InsufficientEtherSupplied(decoded));
            }
            if let Ok(decoded) =
                <Invalid1155BatchTransferEncoding as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::Invalid1155BatchTransferEncoding(decoded));
            }
            if let Ok(decoded) =
                <InvalidBasicOrderParameterEncoding as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::InvalidBasicOrderParameterEncoding(decoded));
            }
            if let Ok(decoded) =
                <InvalidCallToConduit as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidCallToConduit(decoded));
            }
            if let Ok(decoded) =
                <InvalidCanceller as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidCanceller(decoded));
            }
            if let Ok(decoded) =
                <InvalidConduit as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidConduit(decoded));
            }
            if let Ok(decoded) =
                <InvalidERC721TransferAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidERC721TransferAmount(decoded));
            }
            if let Ok(decoded) =
                <InvalidFulfillmentComponentData as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::InvalidFulfillmentComponentData(decoded));
            }
            if let Ok(decoded) =
                <InvalidMsgValue as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidMsgValue(decoded));
            }
            if let Ok(decoded) =
                <InvalidNativeOfferItem as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidNativeOfferItem(decoded));
            }
            if let Ok(decoded) =
                <InvalidProof as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidProof(decoded));
            }
            if let Ok(decoded) =
                <InvalidRestrictedOrder as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidRestrictedOrder(decoded));
            }
            if let Ok(decoded) =
                <InvalidSignature as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidSignature(decoded));
            }
            if let Ok(decoded) =
                <InvalidSigner as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidSigner(decoded));
            }
            if let Ok(decoded) =
                <InvalidTime as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::InvalidTime(decoded));
            }
            if let Ok (decoded) = < MismatchedFulfillmentOfferAndConsiderationComponents as ethers :: core :: abi :: AbiDecode > :: decode (data . as_ref ()) { return Ok (seaportErrors :: MismatchedFulfillmentOfferAndConsiderationComponents (decoded)) }
            if let Ok(decoded) =
                <MissingFulfillmentComponentOnAggregation as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::MissingFulfillmentComponentOnAggregation(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <MissingItemAmount as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::MissingItemAmount(decoded));
            }
            if let Ok(decoded) =
                <MissingOriginalConsiderationItems as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::MissingOriginalConsiderationItems(decoded));
            }
            if let Ok(decoded) = <NoContract as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::NoContract(decoded));
            }
            if let Ok(decoded) =
                <NoReentrantCalls as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::NoReentrantCalls(decoded));
            }
            if let Ok(decoded) =
                <NoSpecifiedOrdersAvailable as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::NoSpecifiedOrdersAvailable(decoded));
            }
            if let Ok(decoded) =
                <OfferAndConsiderationRequiredOnFulfillment as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::OfferAndConsiderationRequiredOnFulfillment(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <OfferCriteriaResolverOutOfRange as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::OfferCriteriaResolverOutOfRange(decoded));
            }
            if let Ok(decoded) =
                <OrderAlreadyFilled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::OrderAlreadyFilled(decoded));
            }
            if let Ok(decoded) =
                <OrderCriteriaResolverOutOfRange as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::OrderCriteriaResolverOutOfRange(decoded));
            }
            if let Ok(decoded) =
                <OrderIsCancelled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::OrderIsCancelled(decoded));
            }
            if let Ok(decoded) =
                <OrderPartiallyFilled as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::OrderPartiallyFilled(decoded));
            }
            if let Ok(decoded) =
                <PartialFillsNotEnabledForOrder as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::PartialFillsNotEnabledForOrder(decoded));
            }
            if let Ok(decoded) =
                <TokenTransferGenericFailure as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::TokenTransferGenericFailure(decoded));
            }
            if let Ok(decoded) =
                <UnresolvedConsiderationCriteria as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportErrors::UnresolvedConsiderationCriteria(decoded));
            }
            if let Ok(decoded) =
                <UnresolvedOfferCriteria as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::UnresolvedOfferCriteria(decoded));
            }
            if let Ok(decoded) =
                <UnusedItemParameters as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportErrors::UnusedItemParameters(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for seaportErrors {
        fn encode(self) -> Vec<u8> {
            match self {
                seaportErrors::BadContractSignature(element) => element.encode(),
                seaportErrors::BadFraction(element) => element.encode(),
                seaportErrors::BadReturnValueFromERC20OnTransfer(element) => element.encode(),
                seaportErrors::BadSignatureV(element) => element.encode(),
                seaportErrors::ConsiderationCriteriaResolverOutOfRange(element) => element.encode(),
                seaportErrors::ConsiderationNotMet(element) => element.encode(),
                seaportErrors::CriteriaNotEnabledForItem(element) => element.encode(),
                seaportErrors::ERC1155BatchTransferGenericFailure(element) => element.encode(),
                seaportErrors::EtherTransferGenericFailure(element) => element.encode(),
                seaportErrors::InexactFraction(element) => element.encode(),
                seaportErrors::InsufficientEtherSupplied(element) => element.encode(),
                seaportErrors::Invalid1155BatchTransferEncoding(element) => element.encode(),
                seaportErrors::InvalidBasicOrderParameterEncoding(element) => element.encode(),
                seaportErrors::InvalidCallToConduit(element) => element.encode(),
                seaportErrors::InvalidCanceller(element) => element.encode(),
                seaportErrors::InvalidConduit(element) => element.encode(),
                seaportErrors::InvalidERC721TransferAmount(element) => element.encode(),
                seaportErrors::InvalidFulfillmentComponentData(element) => element.encode(),
                seaportErrors::InvalidMsgValue(element) => element.encode(),
                seaportErrors::InvalidNativeOfferItem(element) => element.encode(),
                seaportErrors::InvalidProof(element) => element.encode(),
                seaportErrors::InvalidRestrictedOrder(element) => element.encode(),
                seaportErrors::InvalidSignature(element) => element.encode(),
                seaportErrors::InvalidSigner(element) => element.encode(),
                seaportErrors::InvalidTime(element) => element.encode(),
                seaportErrors::MismatchedFulfillmentOfferAndConsiderationComponents(element) => {
                    element.encode()
                }
                seaportErrors::MissingFulfillmentComponentOnAggregation(element) => {
                    element.encode()
                }
                seaportErrors::MissingItemAmount(element) => element.encode(),
                seaportErrors::MissingOriginalConsiderationItems(element) => element.encode(),
                seaportErrors::NoContract(element) => element.encode(),
                seaportErrors::NoReentrantCalls(element) => element.encode(),
                seaportErrors::NoSpecifiedOrdersAvailable(element) => element.encode(),
                seaportErrors::OfferAndConsiderationRequiredOnFulfillment(element) => {
                    element.encode()
                }
                seaportErrors::OfferCriteriaResolverOutOfRange(element) => element.encode(),
                seaportErrors::OrderAlreadyFilled(element) => element.encode(),
                seaportErrors::OrderCriteriaResolverOutOfRange(element) => element.encode(),
                seaportErrors::OrderIsCancelled(element) => element.encode(),
                seaportErrors::OrderPartiallyFilled(element) => element.encode(),
                seaportErrors::PartialFillsNotEnabledForOrder(element) => element.encode(),
                seaportErrors::TokenTransferGenericFailure(element) => element.encode(),
                seaportErrors::UnresolvedConsiderationCriteria(element) => element.encode(),
                seaportErrors::UnresolvedOfferCriteria(element) => element.encode(),
                seaportErrors::UnusedItemParameters(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for seaportErrors {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                seaportErrors::BadContractSignature(element) => element.fmt(f),
                seaportErrors::BadFraction(element) => element.fmt(f),
                seaportErrors::BadReturnValueFromERC20OnTransfer(element) => element.fmt(f),
                seaportErrors::BadSignatureV(element) => element.fmt(f),
                seaportErrors::ConsiderationCriteriaResolverOutOfRange(element) => element.fmt(f),
                seaportErrors::ConsiderationNotMet(element) => element.fmt(f),
                seaportErrors::CriteriaNotEnabledForItem(element) => element.fmt(f),
                seaportErrors::ERC1155BatchTransferGenericFailure(element) => element.fmt(f),
                seaportErrors::EtherTransferGenericFailure(element) => element.fmt(f),
                seaportErrors::InexactFraction(element) => element.fmt(f),
                seaportErrors::InsufficientEtherSupplied(element) => element.fmt(f),
                seaportErrors::Invalid1155BatchTransferEncoding(element) => element.fmt(f),
                seaportErrors::InvalidBasicOrderParameterEncoding(element) => element.fmt(f),
                seaportErrors::InvalidCallToConduit(element) => element.fmt(f),
                seaportErrors::InvalidCanceller(element) => element.fmt(f),
                seaportErrors::InvalidConduit(element) => element.fmt(f),
                seaportErrors::InvalidERC721TransferAmount(element) => element.fmt(f),
                seaportErrors::InvalidFulfillmentComponentData(element) => element.fmt(f),
                seaportErrors::InvalidMsgValue(element) => element.fmt(f),
                seaportErrors::InvalidNativeOfferItem(element) => element.fmt(f),
                seaportErrors::InvalidProof(element) => element.fmt(f),
                seaportErrors::InvalidRestrictedOrder(element) => element.fmt(f),
                seaportErrors::InvalidSignature(element) => element.fmt(f),
                seaportErrors::InvalidSigner(element) => element.fmt(f),
                seaportErrors::InvalidTime(element) => element.fmt(f),
                seaportErrors::MismatchedFulfillmentOfferAndConsiderationComponents(element) => {
                    element.fmt(f)
                }
                seaportErrors::MissingFulfillmentComponentOnAggregation(element) => element.fmt(f),
                seaportErrors::MissingItemAmount(element) => element.fmt(f),
                seaportErrors::MissingOriginalConsiderationItems(element) => element.fmt(f),
                seaportErrors::NoContract(element) => element.fmt(f),
                seaportErrors::NoReentrantCalls(element) => element.fmt(f),
                seaportErrors::NoSpecifiedOrdersAvailable(element) => element.fmt(f),
                seaportErrors::OfferAndConsiderationRequiredOnFulfillment(element) => {
                    element.fmt(f)
                }
                seaportErrors::OfferCriteriaResolverOutOfRange(element) => element.fmt(f),
                seaportErrors::OrderAlreadyFilled(element) => element.fmt(f),
                seaportErrors::OrderCriteriaResolverOutOfRange(element) => element.fmt(f),
                seaportErrors::OrderIsCancelled(element) => element.fmt(f),
                seaportErrors::OrderPartiallyFilled(element) => element.fmt(f),
                seaportErrors::PartialFillsNotEnabledForOrder(element) => element.fmt(f),
                seaportErrors::TokenTransferGenericFailure(element) => element.fmt(f),
                seaportErrors::UnresolvedConsiderationCriteria(element) => element.fmt(f),
                seaportErrors::UnresolvedOfferCriteria(element) => element.fmt(f),
                seaportErrors::UnusedItemParameters(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<BadContractSignature> for seaportErrors {
        fn from(var: BadContractSignature) -> Self {
            seaportErrors::BadContractSignature(var)
        }
    }
    impl ::std::convert::From<BadFraction> for seaportErrors {
        fn from(var: BadFraction) -> Self {
            seaportErrors::BadFraction(var)
        }
    }
    impl ::std::convert::From<BadReturnValueFromERC20OnTransfer> for seaportErrors {
        fn from(var: BadReturnValueFromERC20OnTransfer) -> Self {
            seaportErrors::BadReturnValueFromERC20OnTransfer(var)
        }
    }
    impl ::std::convert::From<BadSignatureV> for seaportErrors {
        fn from(var: BadSignatureV) -> Self {
            seaportErrors::BadSignatureV(var)
        }
    }
    impl ::std::convert::From<ConsiderationCriteriaResolverOutOfRange> for seaportErrors {
        fn from(var: ConsiderationCriteriaResolverOutOfRange) -> Self {
            seaportErrors::ConsiderationCriteriaResolverOutOfRange(var)
        }
    }
    impl ::std::convert::From<ConsiderationNotMet> for seaportErrors {
        fn from(var: ConsiderationNotMet) -> Self {
            seaportErrors::ConsiderationNotMet(var)
        }
    }
    impl ::std::convert::From<CriteriaNotEnabledForItem> for seaportErrors {
        fn from(var: CriteriaNotEnabledForItem) -> Self {
            seaportErrors::CriteriaNotEnabledForItem(var)
        }
    }
    impl ::std::convert::From<ERC1155BatchTransferGenericFailure> for seaportErrors {
        fn from(var: ERC1155BatchTransferGenericFailure) -> Self {
            seaportErrors::ERC1155BatchTransferGenericFailure(var)
        }
    }
    impl ::std::convert::From<EtherTransferGenericFailure> for seaportErrors {
        fn from(var: EtherTransferGenericFailure) -> Self {
            seaportErrors::EtherTransferGenericFailure(var)
        }
    }
    impl ::std::convert::From<InexactFraction> for seaportErrors {
        fn from(var: InexactFraction) -> Self {
            seaportErrors::InexactFraction(var)
        }
    }
    impl ::std::convert::From<InsufficientEtherSupplied> for seaportErrors {
        fn from(var: InsufficientEtherSupplied) -> Self {
            seaportErrors::InsufficientEtherSupplied(var)
        }
    }
    impl ::std::convert::From<Invalid1155BatchTransferEncoding> for seaportErrors {
        fn from(var: Invalid1155BatchTransferEncoding) -> Self {
            seaportErrors::Invalid1155BatchTransferEncoding(var)
        }
    }
    impl ::std::convert::From<InvalidBasicOrderParameterEncoding> for seaportErrors {
        fn from(var: InvalidBasicOrderParameterEncoding) -> Self {
            seaportErrors::InvalidBasicOrderParameterEncoding(var)
        }
    }
    impl ::std::convert::From<InvalidCallToConduit> for seaportErrors {
        fn from(var: InvalidCallToConduit) -> Self {
            seaportErrors::InvalidCallToConduit(var)
        }
    }
    impl ::std::convert::From<InvalidCanceller> for seaportErrors {
        fn from(var: InvalidCanceller) -> Self {
            seaportErrors::InvalidCanceller(var)
        }
    }
    impl ::std::convert::From<InvalidConduit> for seaportErrors {
        fn from(var: InvalidConduit) -> Self {
            seaportErrors::InvalidConduit(var)
        }
    }
    impl ::std::convert::From<InvalidERC721TransferAmount> for seaportErrors {
        fn from(var: InvalidERC721TransferAmount) -> Self {
            seaportErrors::InvalidERC721TransferAmount(var)
        }
    }
    impl ::std::convert::From<InvalidFulfillmentComponentData> for seaportErrors {
        fn from(var: InvalidFulfillmentComponentData) -> Self {
            seaportErrors::InvalidFulfillmentComponentData(var)
        }
    }
    impl ::std::convert::From<InvalidMsgValue> for seaportErrors {
        fn from(var: InvalidMsgValue) -> Self {
            seaportErrors::InvalidMsgValue(var)
        }
    }
    impl ::std::convert::From<InvalidNativeOfferItem> for seaportErrors {
        fn from(var: InvalidNativeOfferItem) -> Self {
            seaportErrors::InvalidNativeOfferItem(var)
        }
    }
    impl ::std::convert::From<InvalidProof> for seaportErrors {
        fn from(var: InvalidProof) -> Self {
            seaportErrors::InvalidProof(var)
        }
    }
    impl ::std::convert::From<InvalidRestrictedOrder> for seaportErrors {
        fn from(var: InvalidRestrictedOrder) -> Self {
            seaportErrors::InvalidRestrictedOrder(var)
        }
    }
    impl ::std::convert::From<InvalidSignature> for seaportErrors {
        fn from(var: InvalidSignature) -> Self {
            seaportErrors::InvalidSignature(var)
        }
    }
    impl ::std::convert::From<InvalidSigner> for seaportErrors {
        fn from(var: InvalidSigner) -> Self {
            seaportErrors::InvalidSigner(var)
        }
    }
    impl ::std::convert::From<InvalidTime> for seaportErrors {
        fn from(var: InvalidTime) -> Self {
            seaportErrors::InvalidTime(var)
        }
    }
    impl ::std::convert::From<MismatchedFulfillmentOfferAndConsiderationComponents> for seaportErrors {
        fn from(var: MismatchedFulfillmentOfferAndConsiderationComponents) -> Self {
            seaportErrors::MismatchedFulfillmentOfferAndConsiderationComponents(var)
        }
    }
    impl ::std::convert::From<MissingFulfillmentComponentOnAggregation> for seaportErrors {
        fn from(var: MissingFulfillmentComponentOnAggregation) -> Self {
            seaportErrors::MissingFulfillmentComponentOnAggregation(var)
        }
    }
    impl ::std::convert::From<MissingItemAmount> for seaportErrors {
        fn from(var: MissingItemAmount) -> Self {
            seaportErrors::MissingItemAmount(var)
        }
    }
    impl ::std::convert::From<MissingOriginalConsiderationItems> for seaportErrors {
        fn from(var: MissingOriginalConsiderationItems) -> Self {
            seaportErrors::MissingOriginalConsiderationItems(var)
        }
    }
    impl ::std::convert::From<NoContract> for seaportErrors {
        fn from(var: NoContract) -> Self {
            seaportErrors::NoContract(var)
        }
    }
    impl ::std::convert::From<NoReentrantCalls> for seaportErrors {
        fn from(var: NoReentrantCalls) -> Self {
            seaportErrors::NoReentrantCalls(var)
        }
    }
    impl ::std::convert::From<NoSpecifiedOrdersAvailable> for seaportErrors {
        fn from(var: NoSpecifiedOrdersAvailable) -> Self {
            seaportErrors::NoSpecifiedOrdersAvailable(var)
        }
    }
    impl ::std::convert::From<OfferAndConsiderationRequiredOnFulfillment> for seaportErrors {
        fn from(var: OfferAndConsiderationRequiredOnFulfillment) -> Self {
            seaportErrors::OfferAndConsiderationRequiredOnFulfillment(var)
        }
    }
    impl ::std::convert::From<OfferCriteriaResolverOutOfRange> for seaportErrors {
        fn from(var: OfferCriteriaResolverOutOfRange) -> Self {
            seaportErrors::OfferCriteriaResolverOutOfRange(var)
        }
    }
    impl ::std::convert::From<OrderAlreadyFilled> for seaportErrors {
        fn from(var: OrderAlreadyFilled) -> Self {
            seaportErrors::OrderAlreadyFilled(var)
        }
    }
    impl ::std::convert::From<OrderCriteriaResolverOutOfRange> for seaportErrors {
        fn from(var: OrderCriteriaResolverOutOfRange) -> Self {
            seaportErrors::OrderCriteriaResolverOutOfRange(var)
        }
    }
    impl ::std::convert::From<OrderIsCancelled> for seaportErrors {
        fn from(var: OrderIsCancelled) -> Self {
            seaportErrors::OrderIsCancelled(var)
        }
    }
    impl ::std::convert::From<OrderPartiallyFilled> for seaportErrors {
        fn from(var: OrderPartiallyFilled) -> Self {
            seaportErrors::OrderPartiallyFilled(var)
        }
    }
    impl ::std::convert::From<PartialFillsNotEnabledForOrder> for seaportErrors {
        fn from(var: PartialFillsNotEnabledForOrder) -> Self {
            seaportErrors::PartialFillsNotEnabledForOrder(var)
        }
    }
    impl ::std::convert::From<TokenTransferGenericFailure> for seaportErrors {
        fn from(var: TokenTransferGenericFailure) -> Self {
            seaportErrors::TokenTransferGenericFailure(var)
        }
    }
    impl ::std::convert::From<UnresolvedConsiderationCriteria> for seaportErrors {
        fn from(var: UnresolvedConsiderationCriteria) -> Self {
            seaportErrors::UnresolvedConsiderationCriteria(var)
        }
    }
    impl ::std::convert::From<UnresolvedOfferCriteria> for seaportErrors {
        fn from(var: UnresolvedOfferCriteria) -> Self {
            seaportErrors::UnresolvedOfferCriteria(var)
        }
    }
    impl ::std::convert::From<UnusedItemParameters> for seaportErrors {
        fn from(var: UnusedItemParameters) -> Self {
            seaportErrors::UnusedItemParameters(var)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "CounterIncremented",
        abi = "CounterIncremented(uint256,address)"
    )]
    pub struct CounterIncrementedFilter {
        pub new_counter: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OrderCancelled",
        abi = "OrderCancelled(bytes32,address,address)"
    )]
    pub struct OrderCancelledFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OrderFulfilled",
        abi = "OrderFulfilled(bytes32,address,address,address,(uint8,address,uint256,uint256)[],(uint8,address,uint256,uint256,address)[])"
    )]
    pub struct OrderFulfilledFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub offer: ::std::vec::Vec<SpentItem>,
        pub consideration: ::std::vec::Vec<ReceivedItem>,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OrderValidated",
        abi = "OrderValidated(bytes32,address,address)"
    )]
    pub struct OrderValidatedFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum seaportEvents {
        CounterIncrementedFilter(CounterIncrementedFilter),
        OrderCancelledFilter(OrderCancelledFilter),
        OrderFulfilledFilter(OrderFulfilledFilter),
        OrderValidatedFilter(OrderValidatedFilter),
    }
    impl ethers::contract::EthLogDecode for seaportEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CounterIncrementedFilter::decode_log(log) {
                return Ok(seaportEvents::CounterIncrementedFilter(decoded));
            }
            if let Ok(decoded) = OrderCancelledFilter::decode_log(log) {
                return Ok(seaportEvents::OrderCancelledFilter(decoded));
            }
            if let Ok(decoded) = OrderFulfilledFilter::decode_log(log) {
                return Ok(seaportEvents::OrderFulfilledFilter(decoded));
            }
            if let Ok(decoded) = OrderValidatedFilter::decode_log(log) {
                return Ok(seaportEvents::OrderValidatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for seaportEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                seaportEvents::CounterIncrementedFilter(element) => element.fmt(f),
                seaportEvents::OrderCancelledFilter(element) => element.fmt(f),
                seaportEvents::OrderFulfilledFilter(element) => element.fmt(f),
                seaportEvents::OrderValidatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `cancel` function with signature `cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])` and selector `[253, 159, 30, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "cancel",
        abi = "cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])"
    )]
    pub struct CancelCall {
        pub orders: ::std::vec::Vec<OrderComponents>,
    }
    #[doc = "Container type for all input parameters for the `fulfillAdvancedOrder` function with signature `fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)` and selector `[231, 172, 171, 36]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "fulfillAdvancedOrder",
        abi = "fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)"
    )]
    pub struct FulfillAdvancedOrderCall {
        pub advanced_order: AdvancedOrder,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub fulfiller_conduit_key: [u8; 32],
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `fulfillAvailableAdvancedOrders` function with signature `fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)` and selector `[135, 32, 27, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "fulfillAvailableAdvancedOrders",
        abi = "fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)"
    )]
    pub struct FulfillAvailableAdvancedOrdersCall {
        pub advanced_orders: ::std::vec::Vec<AdvancedOrder>,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub fulfiller_conduit_key: [u8; 32],
        pub recipient: ethers::core::types::Address,
        pub maximum_fulfilled: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fulfillAvailableOrders` function with signature `fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)` and selector `[237, 152, 165, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "fulfillAvailableOrders",
        abi = "fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)"
    )]
    pub struct FulfillAvailableOrdersCall {
        pub orders: ::std::vec::Vec<Order>,
        pub offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub fulfiller_conduit_key: [u8; 32],
        pub maximum_fulfilled: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fulfillBasicOrder` function with signature `fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))` and selector `[251, 15, 62, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "fulfillBasicOrder",
        abi = "fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))"
    )]
    pub struct FulfillBasicOrderCall {
        pub parameters: BasicOrderParameters,
    }
    #[doc = "Container type for all input parameters for the `fulfillOrder` function with signature `fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)` and selector `[179, 163, 76, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "fulfillOrder",
        abi = "fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)"
    )]
    pub struct FulfillOrderCall {
        pub order: Order,
        pub fulfiller_conduit_key: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getCounter` function with signature `getCounter(address)` and selector `[240, 126, 195, 115]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCounter", abi = "getCounter(address)")]
    pub struct GetCounterCall {
        pub offerer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getOrderHash` function with signature `getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))` and selector `[121, 223, 114, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getOrderHash",
        abi = "getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))"
    )]
    pub struct GetOrderHashCall {
        pub order: OrderComponents,
    }
    #[doc = "Container type for all input parameters for the `getOrderStatus` function with signature `getOrderStatus(bytes32)` and selector `[70, 66, 58, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getOrderStatus", abi = "getOrderStatus(bytes32)")]
    pub struct GetOrderStatusCall {
        pub order_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `incrementCounter` function with signature `incrementCounter()` and selector `[91, 52, 185, 102]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "incrementCounter", abi = "incrementCounter()")]
    pub struct IncrementCounterCall;
    #[doc = "Container type for all input parameters for the `information` function with signature `information()` and selector `[244, 123, 119, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "information", abi = "information()")]
    pub struct InformationCall;
    #[doc = "Container type for all input parameters for the `matchAdvancedOrders` function with signature `matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `[85, 148, 74, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "matchAdvancedOrders",
        abi = "matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])"
    )]
    pub struct MatchAdvancedOrdersCall {
        pub advanced_orders: ::std::vec::Vec<AdvancedOrder>,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub fulfillments: ::std::vec::Vec<Fulfillment>,
    }
    #[doc = "Container type for all input parameters for the `matchOrders` function with signature `matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `[168, 23, 68, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "matchOrders",
        abi = "matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])"
    )]
    pub struct MatchOrdersCall {
        pub orders: ::std::vec::Vec<Order>,
        pub fulfillments: ::std::vec::Vec<Fulfillment>,
    }
    #[doc = "Container type for all input parameters for the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `validate` function with signature `validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])` and selector `[136, 20, 119, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "validate",
        abi = "validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])"
    )]
    pub struct ValidateCall {
        pub orders: ::std::vec::Vec<Order>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum seaportCalls {
        Cancel(CancelCall),
        FulfillAdvancedOrder(FulfillAdvancedOrderCall),
        FulfillAvailableAdvancedOrders(FulfillAvailableAdvancedOrdersCall),
        FulfillAvailableOrders(FulfillAvailableOrdersCall),
        FulfillBasicOrder(FulfillBasicOrderCall),
        FulfillOrder(FulfillOrderCall),
        GetCounter(GetCounterCall),
        GetOrderHash(GetOrderHashCall),
        GetOrderStatus(GetOrderStatusCall),
        IncrementCounter(IncrementCounterCall),
        Information(InformationCall),
        MatchAdvancedOrders(MatchAdvancedOrdersCall),
        MatchOrders(MatchOrdersCall),
        Name(NameCall),
        Validate(ValidateCall),
    }
    impl ethers::core::abi::AbiDecode for seaportCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CancelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::Cancel(decoded));
            }
            if let Ok(decoded) =
                <FulfillAdvancedOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::FulfillAdvancedOrder(decoded));
            }
            if let Ok(decoded) =
                <FulfillAvailableAdvancedOrdersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(seaportCalls::FulfillAvailableAdvancedOrders(decoded));
            }
            if let Ok(decoded) =
                <FulfillAvailableOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::FulfillAvailableOrders(decoded));
            }
            if let Ok(decoded) =
                <FulfillBasicOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::FulfillBasicOrder(decoded));
            }
            if let Ok(decoded) =
                <FulfillOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::FulfillOrder(decoded));
            }
            if let Ok(decoded) =
                <GetCounterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::GetCounter(decoded));
            }
            if let Ok(decoded) =
                <GetOrderHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::GetOrderHash(decoded));
            }
            if let Ok(decoded) =
                <GetOrderStatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::GetOrderStatus(decoded));
            }
            if let Ok(decoded) =
                <IncrementCounterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::IncrementCounter(decoded));
            }
            if let Ok(decoded) =
                <InformationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::Information(decoded));
            }
            if let Ok(decoded) =
                <MatchAdvancedOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::MatchAdvancedOrders(decoded));
            }
            if let Ok(decoded) =
                <MatchOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::MatchOrders(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(seaportCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <ValidateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(seaportCalls::Validate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for seaportCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                seaportCalls::Cancel(element) => element.encode(),
                seaportCalls::FulfillAdvancedOrder(element) => element.encode(),
                seaportCalls::FulfillAvailableAdvancedOrders(element) => element.encode(),
                seaportCalls::FulfillAvailableOrders(element) => element.encode(),
                seaportCalls::FulfillBasicOrder(element) => element.encode(),
                seaportCalls::FulfillOrder(element) => element.encode(),
                seaportCalls::GetCounter(element) => element.encode(),
                seaportCalls::GetOrderHash(element) => element.encode(),
                seaportCalls::GetOrderStatus(element) => element.encode(),
                seaportCalls::IncrementCounter(element) => element.encode(),
                seaportCalls::Information(element) => element.encode(),
                seaportCalls::MatchAdvancedOrders(element) => element.encode(),
                seaportCalls::MatchOrders(element) => element.encode(),
                seaportCalls::Name(element) => element.encode(),
                seaportCalls::Validate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for seaportCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                seaportCalls::Cancel(element) => element.fmt(f),
                seaportCalls::FulfillAdvancedOrder(element) => element.fmt(f),
                seaportCalls::FulfillAvailableAdvancedOrders(element) => element.fmt(f),
                seaportCalls::FulfillAvailableOrders(element) => element.fmt(f),
                seaportCalls::FulfillBasicOrder(element) => element.fmt(f),
                seaportCalls::FulfillOrder(element) => element.fmt(f),
                seaportCalls::GetCounter(element) => element.fmt(f),
                seaportCalls::GetOrderHash(element) => element.fmt(f),
                seaportCalls::GetOrderStatus(element) => element.fmt(f),
                seaportCalls::IncrementCounter(element) => element.fmt(f),
                seaportCalls::Information(element) => element.fmt(f),
                seaportCalls::MatchAdvancedOrders(element) => element.fmt(f),
                seaportCalls::MatchOrders(element) => element.fmt(f),
                seaportCalls::Name(element) => element.fmt(f),
                seaportCalls::Validate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CancelCall> for seaportCalls {
        fn from(var: CancelCall) -> Self {
            seaportCalls::Cancel(var)
        }
    }
    impl ::std::convert::From<FulfillAdvancedOrderCall> for seaportCalls {
        fn from(var: FulfillAdvancedOrderCall) -> Self {
            seaportCalls::FulfillAdvancedOrder(var)
        }
    }
    impl ::std::convert::From<FulfillAvailableAdvancedOrdersCall> for seaportCalls {
        fn from(var: FulfillAvailableAdvancedOrdersCall) -> Self {
            seaportCalls::FulfillAvailableAdvancedOrders(var)
        }
    }
    impl ::std::convert::From<FulfillAvailableOrdersCall> for seaportCalls {
        fn from(var: FulfillAvailableOrdersCall) -> Self {
            seaportCalls::FulfillAvailableOrders(var)
        }
    }
    impl ::std::convert::From<FulfillBasicOrderCall> for seaportCalls {
        fn from(var: FulfillBasicOrderCall) -> Self {
            seaportCalls::FulfillBasicOrder(var)
        }
    }
    impl ::std::convert::From<FulfillOrderCall> for seaportCalls {
        fn from(var: FulfillOrderCall) -> Self {
            seaportCalls::FulfillOrder(var)
        }
    }
    impl ::std::convert::From<GetCounterCall> for seaportCalls {
        fn from(var: GetCounterCall) -> Self {
            seaportCalls::GetCounter(var)
        }
    }
    impl ::std::convert::From<GetOrderHashCall> for seaportCalls {
        fn from(var: GetOrderHashCall) -> Self {
            seaportCalls::GetOrderHash(var)
        }
    }
    impl ::std::convert::From<GetOrderStatusCall> for seaportCalls {
        fn from(var: GetOrderStatusCall) -> Self {
            seaportCalls::GetOrderStatus(var)
        }
    }
    impl ::std::convert::From<IncrementCounterCall> for seaportCalls {
        fn from(var: IncrementCounterCall) -> Self {
            seaportCalls::IncrementCounter(var)
        }
    }
    impl ::std::convert::From<InformationCall> for seaportCalls {
        fn from(var: InformationCall) -> Self {
            seaportCalls::Information(var)
        }
    }
    impl ::std::convert::From<MatchAdvancedOrdersCall> for seaportCalls {
        fn from(var: MatchAdvancedOrdersCall) -> Self {
            seaportCalls::MatchAdvancedOrders(var)
        }
    }
    impl ::std::convert::From<MatchOrdersCall> for seaportCalls {
        fn from(var: MatchOrdersCall) -> Self {
            seaportCalls::MatchOrders(var)
        }
    }
    impl ::std::convert::From<NameCall> for seaportCalls {
        fn from(var: NameCall) -> Self {
            seaportCalls::Name(var)
        }
    }
    impl ::std::convert::From<ValidateCall> for seaportCalls {
        fn from(var: ValidateCall) -> Self {
            seaportCalls::Validate(var)
        }
    }
    #[doc = "Container type for all return fields from the `cancel` function with signature `cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])` and selector `[253, 159, 30, 16]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CancelReturn {
        pub cancelled: bool,
    }
    #[doc = "Container type for all return fields from the `fulfillAdvancedOrder` function with signature `fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)` and selector `[231, 172, 171, 36]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FulfillAdvancedOrderReturn {
        pub fulfilled: bool,
    }
    #[doc = "Container type for all return fields from the `fulfillAvailableAdvancedOrders` function with signature `fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)` and selector `[135, 32, 27, 65]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FulfillAvailableAdvancedOrdersReturn {
        pub available_orders: ::std::vec::Vec<bool>,
        pub executions: ::std::vec::Vec<Execution>,
    }
    #[doc = "Container type for all return fields from the `fulfillAvailableOrders` function with signature `fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)` and selector `[237, 152, 165, 116]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FulfillAvailableOrdersReturn {
        pub available_orders: ::std::vec::Vec<bool>,
        pub executions: ::std::vec::Vec<Execution>,
    }
    #[doc = "Container type for all return fields from the `fulfillBasicOrder` function with signature `fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))` and selector `[251, 15, 62, 225]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FulfillBasicOrderReturn {
        pub fulfilled: bool,
    }
    #[doc = "Container type for all return fields from the `fulfillOrder` function with signature `fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)` and selector `[179, 163, 76, 76]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct FulfillOrderReturn {
        pub fulfilled: bool,
    }
    #[doc = "Container type for all return fields from the `getCounter` function with signature `getCounter(address)` and selector `[240, 126, 195, 115]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCounterReturn {
        pub counter: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `getOrderHash` function with signature `getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))` and selector `[121, 223, 114, 189]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOrderHashReturn {
        pub order_hash: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `getOrderStatus` function with signature `getOrderStatus(bytes32)` and selector `[70, 66, 58, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetOrderStatusReturn {
        pub is_validated: bool,
        pub is_cancelled: bool,
        pub total_filled: ethers::core::types::U256,
        pub total_size: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `incrementCounter` function with signature `incrementCounter()` and selector `[91, 52, 185, 102]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IncrementCounterReturn {
        pub new_counter: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `information` function with signature `information()` and selector `[244, 123, 119, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct InformationReturn {
        pub version: String,
        pub domain_separator: [u8; 32],
        pub conduit_controller: ethers::core::types::Address,
    }
    #[doc = "Container type for all return fields from the `matchAdvancedOrders` function with signature `matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `[85, 148, 74, 66]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MatchAdvancedOrdersReturn {
        pub executions: ::std::vec::Vec<Execution>,
    }
    #[doc = "Container type for all return fields from the `matchOrders` function with signature `matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `[168, 23, 68, 4]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MatchOrdersReturn {
        pub executions: ::std::vec::Vec<Execution>,
    }
    #[doc = "Container type for all return fields from the `name` function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NameReturn {
        pub contract_name: String,
    }
    #[doc = "Container type for all return fields from the `validate` function with signature `validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])` and selector `[136, 20, 119, 50]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ValidateReturn {
        pub validated: bool,
    }
    #[doc = "`AdditionalRecipient(uint256,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AdditionalRecipient {
        pub amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "`AdvancedOrder((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AdvancedOrder {
        pub parameters: OrderParameters,
        pub numerator: u128,
        pub denominator: u128,
        pub signature: ethers::core::types::Bytes,
        pub extra_data: ethers::core::types::Bytes,
    }
    #[doc = "`BasicOrderParameters(address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BasicOrderParameters {
        pub consideration_token: ethers::core::types::Address,
        pub consideration_identifier: ethers::core::types::U256,
        pub consideration_amount: ethers::core::types::U256,
        pub offerer: ethers::core::types::Address,
        pub zone: ethers::core::types::Address,
        pub offer_token: ethers::core::types::Address,
        pub offer_identifier: ethers::core::types::U256,
        pub offer_amount: ethers::core::types::U256,
        pub basic_order_type: u8,
        pub start_time: ethers::core::types::U256,
        pub end_time: ethers::core::types::U256,
        pub zone_hash: [u8; 32],
        pub salt: ethers::core::types::U256,
        pub offerer_conduit_key: [u8; 32],
        pub fulfiller_conduit_key: [u8; 32],
        pub total_original_additional_recipients: ethers::core::types::U256,
        pub additional_recipients: ::std::vec::Vec<AdditionalRecipient>,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`ConsiderationItem(uint8,address,uint256,uint256,uint256,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ConsiderationItem(
        u8,
        ethers::core::types::Address,
        ethers::core::types::U256,
        ethers::core::types::U256,
        ethers::core::types::U256,
        ethers::core::types::Address,
    );
    #[doc = "`CriteriaResolver(uint256,uint8,uint256,uint256,bytes32[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CriteriaResolver {
        pub order_index: ethers::core::types::U256,
        pub side: u8,
        pub index: ethers::core::types::U256,
        pub identifier: ethers::core::types::U256,
        pub criteria_proof: Vec<[u8; 32]>,
    }
    #[doc = "`Execution((uint8,address,uint256,uint256,address),address,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Execution {
        pub item: ReceivedItem,
        pub offerer: ethers::core::types::Address,
        pub conduit_key: [u8; 32],
    }
    #[doc = "`Fulfillment((uint256,uint256)[],(uint256,uint256)[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Fulfillment {
        pub offer_components: ::std::vec::Vec<FulfillmentComponent>,
        pub consideration_components: ::std::vec::Vec<FulfillmentComponent>,
    }
    #[doc = "`FulfillmentComponent(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FulfillmentComponent {
        pub order_index: ethers::core::types::U256,
        pub item_index: ethers::core::types::U256,
    }
    #[doc = "`OfferItem(uint8,address,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OfferItem {
        pub item_type: u8,
        pub token: ethers::core::types::Address,
        pub identifier_or_criteria: ethers::core::types::U256,
        pub start_amount: ethers::core::types::U256,
        pub end_amount: ethers::core::types::U256,
    }
    #[doc = "`Order((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Order {
        pub parameters: OrderParameters,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`OrderComponents(address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OrderComponents {
        pub offerer: ethers::core::types::Address,
        pub zone: ethers::core::types::Address,
        pub offer: ::std::vec::Vec<OfferItem>,
        pub consideration: ::std::vec::Vec<ConsiderationItem>,
        pub order_type: u8,
        pub start_time: ethers::core::types::U256,
        pub end_time: ethers::core::types::U256,
        pub zone_hash: [u8; 32],
        pub salt: ethers::core::types::U256,
        pub conduit_key: [u8; 32],
        pub counter: ethers::core::types::U256,
    }
    #[doc = "`OrderParameters(address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OrderParameters {
        pub offerer: ethers::core::types::Address,
        pub zone: ethers::core::types::Address,
        pub offer: ::std::vec::Vec<OfferItem>,
        pub consideration: ::std::vec::Vec<ConsiderationItem>,
        pub order_type: u8,
        pub start_time: ethers::core::types::U256,
        pub end_time: ethers::core::types::U256,
        pub zone_hash: [u8; 32],
        pub salt: ethers::core::types::U256,
        pub conduit_key: [u8; 32],
        pub total_original_consideration_items: ethers::core::types::U256,
    }
    #[doc = "`ReceivedItem(uint8,address,uint256,uint256,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ReceivedItem {
        pub item_type: u8,
        pub token: ethers::core::types::Address,
        pub identifier: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "`SpentItem(uint8,address,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SpentItem {
        pub item_type: u8,
        pub token: ethers::core::types::Address,
        pub identifier: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
    }
}
