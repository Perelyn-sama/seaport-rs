import { Contract, ethers, providers } from "ethers";
// BigNumberish,
// PayableOverrides,

const offerItemTypeString =
  "OfferItem(uint8 itemType,address token,uint256 identifierOrCriteria,uint256 startAmount,uint256 endAmount)";

console.log(ethers.utils.toUtf8Bytes(offerItemTypeString));

console.log(ethers.utils.toUtf8Bytes("David"));

console.log(ethers.utils.toUtf8Bytes("O"));
