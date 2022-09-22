import { web3 } from "@project-serum/anchor";
import { Wallet } from "@saberhq/solana-contrib";
import { u64 } from "@saberhq/token-utils";
import { fetchPoleCount } from "./accounts";
import {
  createProposal,
  createProposalMeta,
  initPollCount,
} from "./instruction";
import {
  findPollAddress,
  findPollCountAddress,
  findPollMetaAddress,
} from "./pda";

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
): Promise<[web3.Transaction, web3.PublicKey, u64]> => {
  const [pollCountPda, pollCountBump] = await findPollCountAddress();
  const pollCountData = await fetchPoleCount(connection, pollCountPda);
  const index = new u64(pollCountData.proposalCount);
  const [poll, bump] = await findPollAddress(index);

  transaction.add(
    createProposal(connection, wallet, {
      pollCountPda: pollCountPda,
      pollPda: poll,
      pollBump: bump,
    })
  );

  return [transaction, poll, index];
};

export const withCreateProposalMeta = async (
  transaction: web3.Transaction,
  connection: web3.Connection,
  wallet: Wallet,
  params: {
    index: u64;
  }
) => {
  const [pollPda, pollBump] = await findPollAddress(params.index);
  const [pollMetaPda, pollMetaBump] = await findPollMetaAddress(pollPda);

  transaction.add(
    createProposalMeta(connection, wallet, {
      title: "Dummy proposal",
      desctiption: "Dummy description",
      pollPda,
      pollMetaPda,
      pollMetaBump,
    })
  );
};
