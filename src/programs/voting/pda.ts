import * as anchor from "@project-serum/anchor";
import { utils } from "@project-serum/anchor";
import { u64 } from "@saberhq/token-utils";
import { PublicKey } from "@solana/web3.js";

import { MYKHE_ADDRESS } from "../../constants";

/**
 * Fids the PDA of a Governor
 * @param base
 * @returns
 */
export const findPollCountAddress = async (): Promise<[PublicKey, number]> => {
  return await PublicKey.findProgramAddress(
    [anchor.utils.bytes.utf8.encode("my_khe_governor")],
    MYKHE_ADDRESS.Voting
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
    MYKHE_ADDRESS.Voting
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
    MYKHE_ADDRESS.Voting
  );
};
