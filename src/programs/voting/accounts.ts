import { AnchorProvider, BN, Program } from "@project-serum/anchor";
import { SignerWallet } from "@saberhq/solana-contrib";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";

import {
  PollCountData,
  PollData,
  PollMetaData,
  VoteData,
  VOTING_ADDRESS,
  VOTING_IDL,
  VOTING_PROGRAM,
} from "./constants";
import { findPollAddress } from "./pda";

const getProgram = (connection: Connection) => {
  const provider = new AnchorProvider(
    connection,
    new SignerWallet(Keypair.generate()),
    {}
  );
  return new Program<VOTING_PROGRAM>(VOTING_IDL, VOTING_ADDRESS, provider);
};

export const fetchPoleCount = async (
  connection: Connection,
  key: PublicKey
): Promise<PollCountData> => {
  const votingProgram = getProgram(connection);
  return await votingProgram.account.pollCount.fetch(key);
};

export const fetchPoll = async (
  connection: Connection,
  key: PublicKey
): Promise<PollData> => {
  const votingProgram = getProgram(connection);
  return await votingProgram.account.poll.fetch(key);
};

export const fetchPollMeta = async (
  connection: Connection,
  key: PublicKey
): Promise<PollMetaData> => {
  const votingProgram = getProgram(connection);
  return await votingProgram.account.pollMeta.fetch(key);
};

export const fetchVote = async (
  connection: Connection,
  key: PublicKey
): Promise<VoteData> => {
  const votingProgram = getProgram(connection);

  return await votingProgram.account.vote.fetch(key);
};
