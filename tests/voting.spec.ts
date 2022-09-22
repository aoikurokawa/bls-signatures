import * as anchor from "@project-serum/anchor";
import { PublicKey, Transaction } from "@solana/web3.js";
import { expect } from "chai";
import invariant from "tiny-invariant";

import { getProvider, makeSDK, ONE, setupPollCount, ZERO } from "./workspace";
import { findPollAddress, findPollCountAddress } from "../src";
import { withInitPoolCount } from "../src/programs/voting/transaction";
import { expectTXTable } from "@saberhq/chai-solana";
import { SolanaProvider, TransactionEnvelope } from "@saberhq/solana-contrib";
import { fetchPoleCount } from "../src/programs/voting/accounts";

describe("Voting", () => {
  // const sdk = makeSDK();

  // let votingW: VotingWrapper;
  let countDataPda: PublicKey;

  before(async () => {
    // const { votingWrapper } = await setupPollCount({ sdk });
    const provider = getProvider();
    const transaction = new anchor.web3.Transaction();

    [, countDataPda] = await withInitPoolCount(
      transaction,
      provider.connection,
      provider.wallet
    );

    await expectTXTable(
      new TransactionEnvelope(SolanaProvider.init(provider), [
        ...transaction.instructions,
      ]),
      "Create Pole Count"
    ).to.be.fulfilled;

    // votingW = votingWrapper;
  });

  it("PollCounter (Governor) was initialized", async () => {
    const provider = getProvider();
    const pollCountData = await fetchPoleCount(
      provider.connection,
      countDataPda
    );
    const [pollCount, bump] = await findPollCountAddress();

    invariant(pollCountData, "pollcounter data was not loaded");

    expect(pollCountData.bump).to.equal(bump);
    expect(pollCountData.proposalCount.toString()).to.equal(ZERO.toString());

    countDataPda = pollCount;
  });

  // describe("Proposal", () => {
  //   let pollIndex: anchor.BN;
  //   let pollKey: PublicKey;

  //   before("create a dummy poll", async () => {
  //     const { bump, poll, index } = await votingW.createProposal();
  //     await votingW.program.methods
  //       .createPoll(bump, "Dummy poll", "https://www.dummy.com/hello")
  //       .accounts({
  //         countData: countDataPda,
  //         poll: poll,
  //         payer: votingW.provider.wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .rpc();

  //     pollIndex = index;
  //     pollKey = poll;
  //   });

  //   it("Poll as initialized", async () => {
  //     const [expectedPollKey, bump] = await findPollAddress(pollIndex);
  //     expect(pollKey.toString()).to.equal(expectedPollKey.toString());

  //     const pollData = await votingW.findPollByKey(pollKey);
  //     expect(pollData.bump).to.equal(bump);
  //     expect(pollData.index.toString()).to.equal(pollIndex.toString());
  //     expect(pollData.proposer.toString()).to.equal(
  //       votingW.provider.wallet.publicKey.toString()
  //     );
  //   });

  //   it("Vote", async () => {
  //     const { payer, bump, votePdaKey } = await votingW.votePoll(pollKey);

  //     await votingW.program.methods
  //       .votePoll(bump, payer, ONE)
  //       .accounts({
  //         poll: pollKey,
  //         vote: votePdaKey,
  //         payer: votingW.provider.wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //       })
  //       .rpc();

  //     const voteData = await votingW.fetchVote(votePdaKey);

  //     expect(voteData.bump).to.equal(bump);
  //     expect(voteData.optionSelected.toString()).to.equal(ONE.toString());
  //     expect(voteData.poll.toString()).to.equal(pollKey.toString());
  //     expect(voteData.voter.toString()).to.equal(payer.toString());
  //   });
  // });
});
