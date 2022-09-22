// import * as anchor from "@project-serum/anchor";
// import { Keypair, PublicKey, Transaction } from "@solana/web3.js";
// import type * as splToken from "@solana/spl-token";
// import {
//   SignerWallet,
//   SolanaProvider,
//   TransactionEnvelope,
// } from "@saberhq/solana-contrib";
// import { expectTX, expectTXTable } from "@saberhq/chai-solana";

// import { makeSDK, getProvider } from "./workspace";
// import { createMint, createMasterEditionIxs } from "../src/utils";
// import { createStakePool } from "../src/wrappers/locked-voter/locked-voter";

// describe("Create stake pool", () => {
//   const sdk = makeSDK();
//   let stakePoolId: PublicKey;
//   let originalMintTokenAccountId: PublicKey;
//   let originalMint: splToken.Token;
//   const originalMintAuthority = Keypair.generate();

//   before(async () => {
//     const provider = getProvider();
//     // original mint
//     [originalMintTokenAccountId, originalMint] = await createMint(
//       provider.connection,
//       originalMintAuthority,
//       provider.wallet.publicKey,
//       1,
//       originalMintAuthority.publicKey
//     );

//     // master edition
//     const ixs = await createMasterEditionIxs(
//       originalMint.publicKey,
// import { PendingGovernor } from "../../programs/voting/types";
//       originalMintAuthority.publicKey
//     );
//     const txEnvelope = new TransactionEnvelope(
//       SolanaProvider.init({
//         connection: provider.connection,
//         wallet: new SignerWallet(originalMintAuthority),
//         opts: provider.opts,
//       }),
//       ixs
//     );
//     await expectTXTable(txEnvelope, "before", {
//       verbosity: "error",
//       formatLogs: true,
//     }).to.be.fulfilled;
//   });

//   it("Create Pool", async () => {
//     const provider = getProvider();

//     let transaction: Transaction;
//     [transaction, stakePoolId] = await createStakePool(
//       provider.connection,
//       provider.wallet,
//       {}
//     );

//     await expectTXTable(
//       new TransactionEnvelope(SolanaProvider.init(provider), [
//         ...transaction.instructions,
//       ]),
//       "Create pool"
//     ).to.be.fulfilled;
//   });
// });
