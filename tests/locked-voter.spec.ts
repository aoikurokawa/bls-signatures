import * as anchor from "@project-serum/anchor";
import { Keypair } from "@solana/web3.js";

import { makeSDK } from "./workspace";

describe("Locked voter", () => {
  const sdk = makeSDK();

  let base = Keypair.generate();
  let 
});
