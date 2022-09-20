import { web3, Wallet } from "@project-serum/anchor";
import { AnchorProvider } from "@project-serum/anchor/dist/esm";
import { Connection } from "@solana/web3.js";

export const initStakePool = (connection: Connection, wallet: Wallet) => {
  const provider = new AnchorProvider(connection, wallet, {});
};
