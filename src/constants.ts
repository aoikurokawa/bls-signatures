import { PublicKey } from "@solana/web3.js";
import { VotingJSON, VotingProgram } from "./programs";

/**
 * My Khe program types
 */
export interface MyKhePrograms {
  Voting: VotingProgram;
}

export const MYKHE_ADDRESS = {
  Voting: new PublicKey("Fx2TKLRC5V8Xu6R1w42C6k7NUXr35qVudXX86jk6RVky"),
  LockedVoting: new PublicKey("G8BgM1hwZjPWv8jkJhwpj1WKVneuUUuK9QKXDJxJtX2u"),
};

/**
 * Program IDLs
 */
export const MYKHE_IDLS = {
  Voting: VotingJSON,
};

export const preflightCommitment = "recent";
