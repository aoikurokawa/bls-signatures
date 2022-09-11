// import "chai-as-promised";

import type { Idl } from "@project-serum/anchor";
import type {
  PromiseOrValue,
  TransactionLike,
  TransactionReceipt,
} from "@saberhq/solana-contrib";
import { confirmTransactionLike } from "@saberhq/solana-contrib";
import { assert, expect } from "chai";

/**
 * Processes a transaction, expecting rejection or fulfillment.
 *
 * @param tx
 * @param msg
 * @param cb
 * @returns
 */
export const expectTX = (
  tx: PromiseOrValue<TransactionLike | null>,
  msg?: string,
  cb?: (receipt: TransactionReceipt) => Promise<void>
) => {
  const handleReceipt = async (receipt: TransactionReceipt) => {
    await cb?.(receipt);
    return receipt;
  };

  if (tx && "then" in tx) {
    return expect(
      tx
        .then(async (v) => {
          if (v === null) {
            throw new Error("transaction is null");
          }
          return await confirmTransactionLike(v);
        })
        .then(handleReceipt),
      msg
    );
  } else if (tx) {
    return expect(confirmTransactionLike(tx).then(handleReceipt), msg);
  } else {
    return expect(Promise.reject(new Error("transaction is null")), msg);
  }
};

export type IDLError = NonNullable<Idl["errors"]>[number];

export const assertError = (error: IDLError, other: IDLError): void => {
  assert.strictEqual(error.code, other.code);
  assert.strictEqual(error.msg, other.msg);
};
