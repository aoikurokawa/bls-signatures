import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MyKhe } from "../target/types/my_khe";

describe("my-khe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MyKhe as Program<MyKhe>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
