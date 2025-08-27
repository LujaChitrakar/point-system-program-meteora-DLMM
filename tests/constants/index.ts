import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "@coral-xyz/anchor";
import Decimal from "decimal.js";

export const LBCLMM_PROGRAM_IDS = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";

export const ADMIN = "BjjFpCbTrFVn3ZgcdCv4jTLAzbbDCMV1Vo115XJSJ7XG";

export const BASIS_POINT_MAX = 10000;
export const POSITION_FEE = 0.05740608;
export const TOKEN_ACCOUNT_FEE = 0.00203928;
export const BIN_ARRAY_FEE = 0.07143744;
export const POOL_FEE = 0.00718272;
export const BIN_ARRAY_BITMAP_FEE = 0.01180416;
export const BIN_ARRAY_FEE_BN = new BN(
  new Decimal(BIN_ARRAY_FEE).mul(LAMPORTS_PER_SOL).toString()
);
export const POSITION_FEE_BN = new BN(
  new Decimal(POSITION_FEE).mul(LAMPORTS_PER_SOL).toString()
);
export const TOKEN_ACCOUNT_FEE_BN = new BN(
  new Decimal(TOKEN_ACCOUNT_FEE).mul(LAMPORTS_PER_SOL).toString()
);
export const POOL_FEE_BN = new BN(
  new Decimal(POOL_FEE).mul(LAMPORTS_PER_SOL).toString()
);
export const BIN_ARRAY_BITMAP_FEE_BN = new BN(
  new Decimal(BIN_ARRAY_BITMAP_FEE).mul(LAMPORTS_PER_SOL).toString()
);
export const PRECISION = 18446744073709551616;
