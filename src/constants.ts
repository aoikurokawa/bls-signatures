import { PublicKey } from "@solana/web3.js";
import idl from "./idl/voting.json";
import { VotingJSON, VotingProgram } from "./programs";

/**
 * My Khe program types
 */
export interface MyKhePrograms {
  Voting: VotingProgram;
}

export const MYKHE_ADDRESS = {
  Voting: new PublicKey(idl.metadata.address),
};

export const MYKHE_IDLS = {
  Voting: VotingJSON,
};

export const preflightCommitment = "recent";
