import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Voting } from "../target/types/voting";
import { PublicKey } from "@solana/web3.js";
import { programID } from "../src/utils";
import { expect } from "chai";

describe("voting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Voting as Program<Voting>;

  let countDataPda: PublicKey;
  let countDatabump: number;

  beforeEach(async () => {
    const [pda, bump] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("my_khe_governor")],
      programID
    );

    countDataPda = pda;
    countDatabump = bump;
  })

  it("Initialized governor", async () => {
    
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

  })
});
