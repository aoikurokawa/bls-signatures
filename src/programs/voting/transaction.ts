import { web3 } from "@project-serum/anchor";
import { Wallet } from "@saberhq/solana-contrib";
import { u64 } from "@saberhq/token-utils";
import { fetchPoleCount } from "./accounts";
import { createProposal, initPollCount } from "./instruction";
import { findPollAddress, findPollCountAddress } from "./pda";

export const withInitPoolCount = async (
  transaction: web3.Transaction,
  connection: web3.Connection,
  wallet: Wallet
): Promise<[web3.Transaction, web3.PublicKey]> => {
  const [countDataPda, countDataBump] = await findPollCountAddress();

  transaction.add(
    initPollCount(connection, wallet, {
      countDataId: countDataPda,
      bump: countDataBump,
    })
  );

  return [transaction, countDataPda];
};

export const withCreateProposal = async (
  transaction: web3.Transaction,
  connection: web3.Connection,
  wallet: Wallet
): Promise<[web3.Transaction, web3.PublicKey]> => {
  const [countDataPda, countDataBump] = await findPollCountAddress();
  const pollCountData = await fetchPoleCount(connection, countDataPda);
  const index = new u64(pollCountData.proposalCount);
  const [poll, bump] = await findPollAddress(index);
  transaction.add(
    await createProposal(connection, wallet, {
      countDataId: countDataPda,
      pollDataBump: bump,
      pollDataId: poll,
    })
  );

  return [transaction, poll];
};
