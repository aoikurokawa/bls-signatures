import { web3, utils } from "@project-serum/anchor";
import { MYKHE_ADDRESS } from "../../constants";

export const findStakePoolId = async () => {
  return await web3.PublicKey.findProgramAddress(
    [utils.bytes.utf8.encode("my_khe_stake_pool")],
    MYKHE_ADDRESS.LockedVoting
  );
};
