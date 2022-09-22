import * as anchor from "@project-serum/anchor";
import { SolanaProvider } from "@saberhq/solana-contrib";
import { Keypair } from "@solana/web3.js";
import { u64 } from "@saberhq/token-utils";

import { MyKheSDK } from "../../src";
// import { VotingWrapper } from "../../src/wrappers";

export const ZERO = new u64(0);
export const ONE = new u64(1);

// export const makeSDK = (): MyKheSDK => {
//   const anchorProvider = anchor.AnchorProvider.env();
//   anchor.setProvider(anchorProvider);

//   const provider = SolanaProvider.init({
//     connection: anchorProvider.connection,
//     wallet: anchorProvider.wallet,
//     opts: anchorProvider.opts,
//   });

//   return MyKheSDK.load({
//     provider,
//   });
// };

export const getProvider = (): anchor.AnchorProvider => {
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);
  return anchorProvider;
};

// export const setupPollCount = async ({
//   sdk,
// }: {
//   sdk: MyKheSDK;
// }): Promise<{ votingWrapper: VotingWrapper }> => {
//   const baseKP = Keypair.generate();

//   const { wrapper, tx: tx2 } = await sdk.pollCount.createPollCount({ baseKP });
//   // await expectTX(tx2, "create pollcount");

//   return {
//     votingWrapper: wrapper,
//   };
// };
