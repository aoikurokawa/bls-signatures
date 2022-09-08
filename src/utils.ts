import { PublicKey } from "@solana/web3.js";
import idl from "./idl/voting.json";

export const preflightCommitment = "recent";
export const programID = new PublicKey(idl.metadata.address);