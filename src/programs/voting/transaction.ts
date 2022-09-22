import { web3 } from "@project-serum/anchor";
import { Wallet } from "@saberhq/solana-contrib";
import { initPollCount } from "./instruction";
import { findPollCountAddress } from "./pda";

export const withInitPoolCount = async (
  transaction: web3.Transaction,
  connection: web3.Connection,
  wallet: Wallet
): Promise<[web3.Transaction, web3.PublicKey]> => {
  const [countDataPda, countDataBump] = await findPollCountAddress();

  transaction.add(
    initPollCount(connection, wallet, { countDataId: countDataPda })
  );

  return [transaction, countDataPda];
};
