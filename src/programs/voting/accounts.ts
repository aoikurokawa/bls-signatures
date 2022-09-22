import { AnchorProvider, Program } from "@project-serum/anchor";
import { SignerWallet } from "@saberhq/solana-contrib";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";

import {
  VoteData,
  VOTING_ADDRESS,
  VOTING_IDL,
  VOTING_PROGRAM,
} from "./constants";

const getProgram = (connection: Connection) => {
  const provider = new AnchorProvider(
    connection,
    new SignerWallet(Keypair.generate()),
    {}
  );
  return new Program<VOTING_PROGRAM>(VOTING_IDL, VOTING_ADDRESS, provider);
};

export const fetchVote = async (
  connection: Connection,
  key: PublicKey
): Promise<VoteData> => {
  const votingProgram = getProgram(connection);

  return await votingProgram.account.vote.fetch(key);
};
