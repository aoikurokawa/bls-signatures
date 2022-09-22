import { web3, utils } from "@project-serum/anchor";
import { PublicKey } from "@saberhq/solana-contrib";

import { LOCKED_VOTING_ADDRESS } from "./constants";

export const findStakePoolId = async (): Promise<[PublicKey, number]> => {
  return await web3.PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("my_khe_stake_pool")],
    LOCKED_VOTING_ADDRESS
  );
};
