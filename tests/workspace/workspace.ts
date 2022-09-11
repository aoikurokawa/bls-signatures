import * as anchor from "@project-serum/anchor";
import { SolanaProvider } from "@saberhq/solana-contrib";
import { expectTX } from "@saberhq/chai-solana";
import { Keypair, PublicKey } from "@solana/web3.js";

import { MyKheSDK } from "../../src";
import { findPollCountAddress, VotingWrapper } from "../../src/wrappers";
import { MYKHE_ADDRESS } from "../../src";

export const makeSDK = (): MyKheSDK => {
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);

  const provider = SolanaProvider.init({
    connection: anchorProvider.connection,
    wallet: anchorProvider.wallet,
    opts: anchorProvider.opts,
  });

  return MyKheSDK.load({
    provider,
  });
};

export const setupPollCount = async ({
  sdk,
}: {
  sdk: MyKheSDK;
}): Promise<{ votingWrapper: VotingWrapper }> => {
  const baseKP = Keypair.generate();

  const { wrapper, tx: tx2 } = await sdk.pollCount.createPollCount({ baseKP });
  await expectTX(tx2, "create pollcount").to.be.fulfilled;

  return {
    votingWrapper: wrapper,
  };
};
