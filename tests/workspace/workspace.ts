import * as anchor from "@project-serum/anchor";
import { SolanaProvider } from "@saberhq/solana-contrib";
import { Keypair, PublicKey } from "@solana/web3.js";

import { MyKheSDK } from "../../src";
import { findGovernorAddress, VotingWrapper } from "../../src/wrappers";
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

export const setupGovernor = async ({
  sdk,
}: {
  sdk: MyKheSDK;
}): Promise<{ votingWrapper: VotingWrapper }> => {
  const baseKP = Keypair.generate();
  const [pollCount] = await findGovernorAddress();

  const { wrapper, tx: tx2 } = await sdk.govern.createGovernor({ baseKP });

  return {
    votingWrapper: wrapper,
  };
};
