import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {} from "@saberhq/chai-solana";
import { PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";

import { Voting } from "../target/types/voting";
import { MYKHE_ADDRESS } from "../src/constants";
import { makeSDK, setupPollCount } from "./workspace";
import { findPollCountAddress, VotingWrapper } from "../src";

describe("Voting", () => {
  const sdk = makeSDK();

  let votingW: VotingWrapper;
  let countDataPda: PublicKey;
  let countDatabump: number;

  before(async () => {
    const { votingWrapper } = await setupPollCount({ sdk });

    votingW = votingWrapper;
  });

  it("PollCounter (Governor) was initialized", async () => {
    const pollCountData = await votingW.data();
    const [pollCount, bump] = await findPollCountAddress();
    expect(votingW.pollCountKey).to.eqAddress(pollCount);
  });

  // it("Create dummy poll", async () => {
  //   await program.methods
  //     .createPoll("Dummy poll", options)
  //     .accounts({
  //       countData: countDataPda,
  //       payer: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //     })
  //     .rpc();
  // });
});
