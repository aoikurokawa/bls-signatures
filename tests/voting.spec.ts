import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import {} from "@saberhq/chai-solana";
import { PublicKey } from "@solana/web3.js";
import { assert, expect } from "chai";
import invariant from "tiny-invariant";

import { Voting } from "../target/types/voting";
import { MYKHE_ADDRESS } from "../src/constants";
import { makeSDK, setupPollCount, ZERO } from "./workspace";
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
    expect(votingW.pollCountKey.toString()).to.eq(pollCount.toString());

    invariant(pollCountData, "pollcounter data was not loaded");

    expect(pollCountData.bump).to.equal(bump);
    expect(pollCountData.proposalCount.toString()).to.equal(ZERO.toString());
  });

  describe("Proposal", () => {
    let pollIndex: anchor.BN;
    let pollKey: PublicKey;

    beforeEach("create a dummy poll", async () => {
      const { poll, index, tx } = await votingW.createProposal(
        "Dummy title",
        "https://www.dummy.com/hello"
      );
      await votingW.program.methods
        .createPoll("Dummy poll", "https://www.dummy.com/hello")
        .accounts({
          countData: countDataPda,
          poll: poll,
          payer: votingW.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      pollIndex = index;
      pollKey = poll;
    });
  });
});
