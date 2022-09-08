import { PublicKey } from "@solana/web3.js";
import { PollCountData } from "../../programs/voting";
import { MyKheSDK } from "../../sdk";

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
}
