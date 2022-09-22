import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";

export type Poll = {
  bump: number;
  poll: PublicKey;
  index: BN;
  // tx: TransactionEnvelope;
};

export type Vote = {
  payer: PublicKey;
  bump: number;
  votePdaKey: PublicKey;
};

/**
 * Side of a vote
 */
export enum VoteSide {
  /**
   * Vote against the passing of the proposal
   */
  Against = 1,
  /**
   * Vote to make the proposal pass
   */
  For = 2,
}

/**
 * Labels for vote side
 */
export const VOTE_SIDE_LABELS: { [k in VoteSide]: string } = {
  [VoteSide.For]: "For",
  [VoteSide.Against]: "Against",
};
