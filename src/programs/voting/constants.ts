import { PublicKey } from "@solana/web3.js";

import * as VOTING_TYPES from "../../idl/voting";

export const VOTING_ADDRESS = new PublicKey(
  "Fx2TKLRC5V8Xu6R1w42C6k7NUXr35qVudXX86jk6RVky"
);

export type VOTING_PROGRAM = VOTING_TYPES.Voting;

export const VOTING_IDL = VOTING_TYPES.IDL;
