import type { Wallet } from "@saberhq/solana-contrib";
import { web3, Program } from "@project-serum/anchor";
import { AnchorProvider } from "@project-serum/anchor/dist/esm";
import {
  Connection,
  TransactionInstruction,
  SystemProgram,
} from "@solana/web3.js";

import {
  STAKE_POOL_PROGRAM,
  STAKE_POOL_IDL,
  LOCKED_VOTING_ADDRESS,
} from "./constants";
import { findStakePoolId } from "./pda";

export const initStakePool = async (
  connection: Connection,
  wallet: Wallet
): Promise<web3.TransactionInstruction> => {
  const provider = new AnchorProvider(connection, wallet, {});
  const [poolPda, poolBump] = await findStakePoolId();
  const stakePoolProgram = new Program<STAKE_POOL_PROGRAM>(
    STAKE_POOL_IDL,
    LOCKED_VOTING_ADDRESS,
    provider
  );

  return stakePoolProgram.instruction.initPool(poolBump, {
    accounts: {
      stakePool: poolPda,
      payer: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
  });
};
