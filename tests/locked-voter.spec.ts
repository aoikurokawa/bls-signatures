import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey } from "@solana/web3.js";
import type * as splToken from "@solana/spl-token";

import { makeSDK, getProvider } from "./workspace";
import { createMint } from "../src/utils";

describe("Create stake pool", () => {
  const sdk = makeSDK();
  let stakePoolId: PublicKey;
  let originalMintTokenAccountId: PublicKey;
  let originalMint: splToken.Token;
  const originalMintAuthority = Keypair.generate();

  before(async () => {
    const provider = getProvider();
    [originalMintTokenAccountId, originalMint] = await createMint(
      provider.connection,
      originalMintAuthority,
      provider.wallet.publicKey,
      1,
      originalMintAuthority.publicKey
    );

    // const ixs = await 
  });

  let base = Keypair.generate();
});
