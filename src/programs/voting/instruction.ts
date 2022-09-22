import { AnchorProvider, Program } from "@project-serum/anchor";
import { Wallet } from "@saberhq/solana-contrib";
import {
  Connection,
  SystemProgram,
  TransactionInstruction,
  PublicKey,
} from "@solana/web3.js";
import { VOTING_ADDRESS, VOTING_IDL, VOTING_PROGRAM } from "./constants";

export const initPollCount = (
  connection: Connection,
  wallet: Wallet,
  params: {
    countDataId: PublicKey;
  }
): TransactionInstruction => {
  const provider = new AnchorProvider(connection, wallet, {});
  const votingProgram = new Program<VOTING_PROGRAM>(
    VOTING_IDL,
    VOTING_ADDRESS,
    provider
  );

  return votingProgram.instruction.initialize({
    accounts: {
      countData: params.countDataId,
      payer: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
  });
};
