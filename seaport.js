import { Contract, ethers, providers, utils } from "ethers";
// BigNumberish,
// PayableOverrides,

const offerItemTypeString =
  "OfferItem(uint8 itemType,address token,uint256 identifierOrCriteria,uint256 startAmount,uint256 endAmount)";

// console.log(ethers.utils.toUtf8Bytes(offerItemTypeString));

const offerItemTypeHash = ethers.utils.keccak256(
    ethers.utils.toUtf8Bytes(offerItemTypeString)
);

// console.log(offerItemTypeHash);
console.log(utils.keccak256(utils.toUtf8Bytes("Perelyn")))
console.log(utils.toUtf8Bytes("Perelyn"));