import type { Wallet } from "@saberhq/solana-contrib";
import { web3 } from "@project-serum/anchor";
import { Transaction, Connection, PublicKey } from "@solana/web3.js";
import { withInitStakePool } from "./transaction";

export const createStakePool = async (
  connection: Connection,
  wallet: Wallet,
  params: {}
): Promise<[Transaction, PublicKey]> => {
  const transaction = new Transaction();

  const [, stakePoolId] = await withInitStakePool(
    transaction,
    connection,
    wallet
  );

  return [transaction, stakePoolId];
};
