import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

import { Voting } from "../target/types/voting";
import { MYKHE_ADDRESS } from "../src/constants";
import { makeSDK } from "./workspace";
import { VotingWrapper } from "../src";

describe("Voting", () => {
  const sdk = makeSDK();

  let votingWrapper: VotingWrapper;
  let countDataPda: PublicKey;
  let countDatabump: number;

  before(async () => {
    const [pda, bump] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("my_khe_governor")],
      MYKHE_ADDRESS.Voting
    );

    countDataPda = pda;
    countDatabump = bump;
  });

  it("Initialized poll counter", async () => {
    await program.methods
      .initialize(countDatabump)
      .accounts({
        countData: countDataPda,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.pollCount.fetch(countDataPda);

    expect(account.bump).to.equal(countDatabump);
    expect(account.proposalCount).to.equal(0);
  });

  it("Create dummy poll", async () => {
    await program.methods
      .createPoll("Dummy poll", options)
      .accounts({
        countData: countDataPda,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
  });
});
