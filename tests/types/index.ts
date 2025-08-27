import {
  BN,
  BorshAccountsCoder,
  IdlAccounts,
  IdlTypes,
  Program,
  ProgramAccount,
} from "@coral-xyz/anchor";
import { LbClmm } from "../idl";
import {
  AccountMeta,
  Keypair,
  PublicKey,
  TransactionInstruction,
} from "@solana/web3.js";
import Decimal from "decimal.js";
import { Mint } from "@solana/spl-token";
import { getPriceOfBinByBinId } from "../helpers";

export const POSITION_MIN_SIZE = 8112;
export const POSITION_BIN_DATA_SIZE = 112;

export interface BinAndAmount {
  binId: number;
  xAmountBpsOfTotal: BN;
  yAmountBpsOfTotal: BN;
}

export type ClmmProgram = Program<LbClmm>;

export type LbPair = IdlAccounts<LbClmm>["lbPair"];
export type LbPairAccount = ProgramAccount<IdlAccounts<LbClmm>["lbPair"]>;

export type Position = IdlAccounts<LbClmm>["position"];

export interface GetOrCreateATAResponse {
  ataPubKey: PublicKey;
  ix?: TransactionInstruction;
}

export interface TokenReserve {
  publicKey: PublicKey;
  reserve: PublicKey;
  mint: Mint;
  amount: bigint;
  owner: PublicKey;
  transferHookAccountMetas: AccountMeta[];
}

export type Bin = IdlTypes<LbClmm>["bin"];

export type RebalanceAddLiquidityParam = IdlTypes<LbClmm>["addLiquidityParams"];
export type RebalanceRemoveLiquidityParam =
  IdlTypes<LbClmm>["removeLiquidityParams"];

export type BinArrayBitmapExtensionAccount = ProgramAccount<
  IdlAccounts<LbClmm>["binArrayBitmapExtension"]
>;
export type BinArrayBitmapExtension =
  IdlAccounts<LbClmm>["binArrayBitmapExtension"];

export interface TInitializePositionAndAddLiquidityParams {
  positionPubKey: PublicKey;
  totalXAmount: BN;
  totalYAmount: BN;
  xYAmountDistribution: BinAndAmount[];
  user: PublicKey;
  slippage?: number;
}

export interface BinLiquidity {
  binId: number;
  xAmount: BN;
  yAmount: BN;
  supply: BN;
  version: number;
  price: string;
  pricePerToken: string;
  feeAmountXPerTokenStored: BN;
  feeAmountYPerTokenStored: BN;
  rewardPerTokenStored: BN[];
}

export module BinLiquidity {
  export function fromBin(
    bin: Bin,
    binId: number,
    binStep: number,
    baseTokenDecimal: number,
    quoteTokenDecimal: number,
    version: number
  ): BinLiquidity {
    const pricePerLamport = getPriceOfBinByBinId(binId, binStep).toString();
    return {
      binId,
      xAmount: bin.amountX,
      yAmount: bin.amountY,
      supply: bin.liquiditySupply,
      price: pricePerLamport,
      version,
      pricePerToken: new Decimal(pricePerLamport)
        .mul(new Decimal(10 ** (baseTokenDecimal - quoteTokenDecimal)))
        .toString(),
      feeAmountXPerTokenStored: bin.feeAmountXPerTokenStored,
      feeAmountYPerTokenStored: bin.feeAmountYPerTokenStored,
      rewardPerTokenStored: bin.rewardPerTokenStored,
    };
  }

  export function empty(
    binId: number,
    binStep: number,
    baseTokenDecimal: number,
    quoteTokenDecimal: number,
    version: number
  ): BinLiquidity {
    const pricePerLamport = getPriceOfBinByBinId(binId, binStep).toString();
    return {
      binId,
      xAmount: new BN(0),
      yAmount: new BN(0),
      supply: new BN(0),
      price: pricePerLamport,
      version,
      pricePerToken: new Decimal(pricePerLamport)
        .mul(new Decimal(10 ** (baseTokenDecimal - quoteTokenDecimal)))
        .toString(),
      feeAmountXPerTokenStored: new BN(0),
      feeAmountYPerTokenStored: new BN(0),
      rewardPerTokenStored: [new BN(0), new BN(0)],
    };
  }
}

export interface PositionBinData {
  binId: number;
  price: string;
  pricePerToken: string;
  binXAmount: string;
  binYAmount: string;
  binLiquidity: string;
  positionLiquidity: string;
  positionXAmount: string;
  positionYAmount: string;
  positionFeeXAmount: string;
  positionFeeYAmount: string;
  positionRewardAmount: string[];
}

export interface PositionData {
  totalXAmount: string;
  totalYAmount: string;
  positionBinData: PositionBinData[];
  lastUpdatedAt: BN;
  upperBinId: number;
  lowerBinId: number;
  feeX: BN;
  feeY: BN;
  rewardOne: BN;
  rewardTwo: BN;
  feeOwner: PublicKey;
  totalClaimedFeeXAmount: BN;
  totalClaimedFeeYAmount: BN;
  feeXExcludeTransferFee: BN;
  feeYExcludeTransferFee: BN;
  rewardOneExcludeTransferFee: BN;
  rewardTwoExcludeTransferFee: BN;
  totalXAmountExcludeTransferFee: BN;
  totalYAmountExcludeTransferFee: BN;
  owner: PublicKey;
}
