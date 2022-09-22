import { AnchorProvider, Program } from "@project-serum/anchor";
import { Wallet } from "@saberhq/solana-contrib";
import {
  Connection,
  SystemProgram,
  TransactionInstruction,
  PublicKey,
} from "@solana/web3.js";

import { VOTING_ADDRESS, VOTING_IDL, VOTING_PROGRAM } from "./constants";

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

export const createProposal = (
  connection: Connection,
  wallet: Wallet,
  params: {
    pollCountPda: PublicKey;
    pollPda: PublicKey;
    pollBump: number;
  }
): TransactionInstruction => {
  const provider = new AnchorProvider(connection, wallet, {});
  const votingProgram = new Program<VOTING_PROGRAM>(
    VOTING_IDL,
    VOTING_ADDRESS,
    provider
  );

  return votingProgram.instruction.createPoll(params.pollBump, {
    accounts: {
      countData: params.pollCountPda,
      poll: params.pollPda,
      payer: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
  });
};

export const createProposalMeta = (
  connection: Connection,
  wallet: Wallet,
  params: {
    pollMetaBump: number;
    title: string;
    desctiptionLink: string;
    pollPda: PublicKey;
    pollMetaPda: PublicKey;
  }
) => {
  const provider = new AnchorProvider(connection, wallet, {});
  const votingProgram = new Program<VOTING_PROGRAM>(
    VOTING_IDL,
    VOTING_ADDRESS,
    provider
  );

  return votingProgram.instruction.createPollMeta(
    params.pollMetaBump,
    params.title,
    params.desctiptionLink,
    {
      accounts: {
        pollMeta: params.pollMetaPda,
        poll: params.pollPda,
        payer: wallet.publicKey,
        proposer: wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
    }
  );
};
