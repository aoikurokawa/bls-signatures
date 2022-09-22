import { AnchorProvider, Program } from "@project-serum/anchor";
import { Wallet } from "@saberhq/solana-contrib";
import {
  Connection,
  SystemProgram,
  TransactionInstruction,
  PublicKey,
} from "@solana/web3.js";
import { u64 } from "@saberhq/token-utils";

import { VOTING_ADDRESS, VOTING_IDL, VOTING_PROGRAM } from "./constants";
import { findPollAddress } from "./pda";
import { fetchPoleCount } from "./accounts";

export const initPollCount = (
  connection: Connection,
  wallet: Wallet,
  params: {
    countDataId: PublicKey;
    bump: number;
  }
): TransactionInstruction => {
  const provider = new AnchorProvider(connection, wallet, {});
  const votingProgram = new Program<VOTING_PROGRAM>(
    VOTING_IDL,
    VOTING_ADDRESS,
    provider
  );

  return votingProgram.instruction.initialize(params.bump, {
    accounts: {
      countData: params.countDataId,
      payer: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
  });
};

export const createProposal = async (
  connection: Connection,
  wallet: Wallet,
  params: {
    countDataId: PublicKey;
    pollDataId: PublicKey;
    pollDataBump: number;
  }
): Promise<TransactionInstruction> => {
  const provider = new AnchorProvider(connection, wallet, {});
  const votingProgram = new Program<VOTING_PROGRAM>(
    VOTING_IDL,
    VOTING_ADDRESS,
    provider
  );

  return votingProgram.instruction.createPoll(params.pollDataBump, {
    accounts: {
      countData: params.countDataId,
      poll: params.pollDataId,
      payer: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
  });
};
