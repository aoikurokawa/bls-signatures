import {
  PublicKey,
  TransactionInstruction,
  SystemProgram,
} from "@solana/web3.js";
import type BN from "bn.js";
import { u64 } from "@saberhq/token-utils";

import { PollCountData, PollData } from "../../programs/voting";
import { MyKheSDK } from "../../sdk";
import { findPollAddress } from "./pda";

export class PollCountDataWrapper {
  private _pollCount: PollCountData | null = null;

  constructor(readonly sdk: MyKheSDK, readonly pollCountKey: PublicKey) {}

  get provider() {
    return this.sdk.provider;
  }

  get program() {
    return this.sdk.program;
  }

  async reload(): Promise<PollCountData> {
    return await this.program.Voting.account.pollCountData.fetch(
      this.pollCountKey
    );
  }

  async data(): Promise<PollCountData> {
    if (!this._pollCount) {
      this._pollCount = await this.reload();
    }
    return this._pollCount;
  }

  async findPollAddress(index: BN): Promise<PublicKey> {
    const [key] = await findPollAddress(index);
    return key;
  }

  async findPollByKey(key: PublicKey): Promise<PollData> {
    return await this.program.Voting.account.pollData.fetch(key);
  }

  async fetchPoll(index: BN): Promise<PollData> {
    const key = await this.findPollAddress(index);
    return await this.findPollByKey(key);
  }

  /**
   * Creates a new poll
   * @returns
   */
  async createProposal({
    proposer = this.sdk.provider.wallet.publicKey,
  }: {
    proposer?: PublicKey;
  }) {
    const { provider } = this.sdk;

    const pollCountData = await this.reload();
    const index = new u64(pollCountData.proposalCount);
    const [poll, bump] = await findPollAddress(index);

    const ixs: TransactionInstruction[] = [];

    ixs.push(
      this.sdk.program.Voting.instruction.createPoll("Dummy Poll", {
        accounts: {
          countData: this.pollCountKey,
          poll: poll,
          payer: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
      })
    );

    return {
      poll,
      index,
    };
  }
}
