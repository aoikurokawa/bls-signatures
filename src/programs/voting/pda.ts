import * as anchor from "@project-serum/anchor";
import { u64 } from "@saberhq/token-utils";
import { PublicKey } from "@solana/web3.js";

import { VOTING_ADDRESS } from "./constants";

/**
 * Fids the PDA of a Governor
 * @param base
 * @returns
 */
export const findPollCountAddress = async (): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode("my_khe_governor")],
    VOTING_ADDRESS
  );
};

/**
 * Finds the PDA of a Proposal
 */
export const findPollAddress = async (
  index: u64
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode("my_khe_proposal"),
      index.toArrayLike(Buffer, "le", 8),
    ],
    VOTING_ADDRESS
  );
};

/**
 * Finds the PDA of a Proposal PDA
 * @param pollPda
 * @returns
 */
export const findPollMetaAddress = async (
  pollPda: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode("my_khe_proposal_meta"),
      pollPda.toBuffer(),
    ],
    VOTING_ADDRESS
  );
};

/**
 * Finds the PDA of a Vote
 * @param proposalKey
 * @param voterKey
 * @returns
 */
export const findVoteAddress = async (
  pollKey: PublicKey,
  voter: PublicKey
): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [
      anchor.utils.bytes.utf8.encode("my_khe_vote"),
      pollKey.toBuffer(),
      voter.toBuffer(),
    ],
    VOTING_ADDRESS
  );
};
