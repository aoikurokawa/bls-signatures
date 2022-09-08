import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Voting } from "../target/types/voting";
import { PublicKey } from "@solana/web3.js";
import { programID } from "../src/utils";

describe("voting", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Voting as Program<Voting>;

  it("Is initialized!", async () => {
    const [countDataPda, bump] = await PublicKey.findProgramAddress(
      [anchor.utils.bytes.utf8.encode("my_khe_governor")],
      programID
    );
    const tx = await program.methods
      .initialize(bump)
      .accounts({
        countData: countDataPda,
        payer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });
});
