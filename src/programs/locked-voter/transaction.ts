import type { Wallet } from "@saberhq/solana-contrib";
import { web3 } from "@project-serum/anchor";
import { initStakePool } from "./instruction";
import { findStakePoolId } from "./pda";

export const withInitStakePool = async (
  transaction: web3.Transaction,
  connection: web3.Connection,
  wallet: Wallet
): Promise<[web3.Transaction, web3.PublicKey]> => {
  const [stakePoolId] = await findStakePoolId();
  transaction.add(await initStakePool(connection, wallet));

  return [transaction, stakePoolId];
};
