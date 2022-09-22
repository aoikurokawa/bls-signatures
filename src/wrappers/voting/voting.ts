import {
  PublicKey,
  TransactionInstruction,
  SystemProgram,
  Keypair,
} from "@solana/web3.js";
import type BN from "bn.js";
import { u64 } from "@saberhq/token-utils";

import { PollCountData, PollData, VoteData } from "../../programs/voting";
import { MyKheSDK } from "../../sdk";
import {
  findPollCountAddress,
  findPollAddress,
  findVoteAddress,
} from "../../programs/voting/pda";
import { Poll, Vote } from "../../programs/voting/types";

// export class VotingWrapper {
//   private _pollCount: PollCountData | null = null;

//   constructor(readonly sdk: MyKheSDK, readonly pollCountKey: PublicKey) {}

//   get provider() {
//     return this.sdk.provider;
//   }

//   get program() {
//     return this.sdk.program.Voting;
//   }

//   async reload(): Promise<PollCountData> {
//     const [pollCountData, bump] = await findPollCountAddress();
//     return await this.program.account.pollCount.fetch(pollCountData);
//   }

//   async data(): Promise<PollCountData> {
//     if (!this._pollCount) {
//       this._pollCount = await this.reload();
//     }
//     return this._pollCount;
//   }

//   async findPollAddress(index: BN): Promise<PublicKey> {
//     const [key] = await findPollAddress(index);
//     return key;
//   }

//   async findPollByKey(key: PublicKey): Promise<PollData> {
//     return await this.program.account.poll.fetch(key);
//   }

//   async fetchPoll(index: BN): Promise<PollData> {
//     const key = await this.findPollAddress(index);
//     return await this.findPollByKey(key);
//   }

//   async findVoteKeyAndBump(pollkey: PublicKey): Promise<[PublicKey, number]> {
//     const voter = this.provider.wallet.publicKey;
//     return await findVoteAddress(pollkey, voter);
//   }

//   async fetchVote(pollKey: PublicKey): Promise<VoteData> {
//     return await this.program.account.vote.fetch(pollKey);
//   }

//   /**
//    * Creates a new poll
//    * @returns
//    */
//   async createProposal(): Promise<Poll> {
//     const pollCountData = await this.reload();
//     const index = new u64(pollCountData.proposalCount);
//     const [poll, bump] = await findPollAddress(index);

//     return {
//       bump,
//       poll,
//       index,
//     };
//   }

//   /**
//    * Vote
//    * @returns Promise<Vote>
//    */
//   async votePoll(pollKey: PublicKey): Promise<Vote> {
//     const { provider } = this.sdk;
//     const [votePdaKey, voteBump] = await findVoteAddress(
//       pollKey,
//       provider.wallet.publicKey
//     );

//     return {
//       payer: provider.wallet.publicKey,
//       bump: voteBump,
//       votePdaKey,
//     };
//   }
// }
