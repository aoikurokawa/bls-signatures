import type { BN } from "@project-serum/anchor";
import { newProgramMap } from "@saberhq/anchor-contrib";
import type { AugmentedProvider, Provider } from "@saberhq/solana-contrib";
import {
  SolanaAugmentedProvider,
  TransactionEnvelope,
} from "@saberhq/solana-contrib";
import type { PublicKey, Signer } from "@solana/web3.js";
import { SystemProgram } from "@solana/web3.js";

import type { MyKhePrograms } from "./constants";
import { MYKHE_ADDRESS, MYKHE_IDLS } from "./constants";
import { PollCountWrapper } from "./wrappers/voting/pollCount";

/**
 * MyKheSDK
 */
export class MyKheSDK {
  constructor(
    readonly provider: AugmentedProvider,
    readonly program: MyKhePrograms
  ) {}

  /**
   * Creates a new instance of the SDK with the given keypair
   * @param signer
   * @returns
   */
  withSigner(signer: Signer): MyKheSDK {
    return MyKheSDK.load({
      provider: this.provider.withSigner(signer),
    });
  }

  /**
   *  laods the SDK
   * @param provider
   * @returns
   */
  static load({ provider }: { provider: Provider }): MyKheSDK {
    const programs: MyKhePrograms = newProgramMap<MyKhePrograms>(
      provider,
      MYKHE_IDLS,
      MYKHE_ADDRESS
    );
    return new MyKheSDK(new SolanaAugmentedProvider(provider), programs);
  }

  /**
   * PollCount program helpers
   */
  get pollCount(): PollCountWrapper {
    return new PollCountWrapper(this);
  }
}
