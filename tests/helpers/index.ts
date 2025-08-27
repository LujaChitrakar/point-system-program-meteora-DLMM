import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  NATIVE_MINT,
  TOKEN_PROGRAM_ID,
  TokenAccountNotFoundError,
  TokenInvalidAccountOwnerError,
  createAssociatedTokenAccountIdempotentInstruction,
  getAccount,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { Connection, PublicKey } from "@solana/web3.js";
import { GetOrCreateATAResponse } from "../types";
import Decimal from "decimal.js";
import { BASIS_POINT_MAX } from "../constants";

export function getPriceOfBinByBinId(binId: number, binStep: number): Decimal {
  const binStepNum = new Decimal(binStep).div(new Decimal(BASIS_POINT_MAX));
  return new Decimal(1).add(new Decimal(binStepNum)).pow(new Decimal(binId));
}
export async function getTokenBalance(
  conn: Connection,
  tokenAccount: PublicKey
): Promise<bigint> {
  const acc = await getAccount(conn, tokenAccount);
  return acc.amount;
}

export const getOrCreateATAInstruction = async (
  connection: Connection,
  tokenMint: PublicKey,
  owner: PublicKey,
  programId?: PublicKey,
  payer: PublicKey = owner,
  allowOwnerOffCurve = true
): Promise<GetOrCreateATAResponse> => {
  programId = programId ?? TOKEN_PROGRAM_ID;
  const toAccount = getAssociatedTokenAddressSync(
    tokenMint,
    owner,
    allowOwnerOffCurve,
    programId,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  try {
    await getAccount(connection, toAccount, connection.commitment, programId);

    return { ataPubKey: toAccount, ix: undefined };
  } catch (e) {
    if (
      e instanceof TokenAccountNotFoundError ||
      e instanceof TokenInvalidAccountOwnerError
    ) {
      const ix = createAssociatedTokenAccountIdempotentInstruction(
        payer,
        toAccount,
        owner,
        tokenMint,
        programId,
        ASSOCIATED_TOKEN_PROGRAM_ID
      );

      return { ataPubKey: toAccount, ix };
    } else {
      /* handle error */
      console.error("Error::getOrCreateATAInstruction", e);
      throw e;
    }
  }
};
