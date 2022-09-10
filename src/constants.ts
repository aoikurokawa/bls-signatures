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
  Voting: new PublicKey("AAFLi32KG5XDdfcJUEjHnQFCghQfusX4pbqtYaivv7DE"),
};

/**
 * Program IDLs
 */
export const MYKHE_IDLS = {
  Voting: VotingJSON,
};

export const preflightCommitment = "recent";
