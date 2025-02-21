import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Batch } from "../target/types/batch";
import { expect } from "chai";

describe("batch", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Batch as Program<Batch>;
  const wallet = provider.wallet as anchor.Wallet;
  it("Is initialized!", async () => {
    const accKey = web3.Keypair.generate();
    // init transaction
    const initTx = await program.methods.initialize().accounts({
      pda: accKey.publicKey,
    })
      .transaction();

    // console.log(initTx);

    // set transaction
    const setTx = await program.methods.set(1024).accounts({
      pda: accKey.publicKey,
    })
      .transaction();

    // combine the two transactions
    const tx = initTx.add(setTx);

    // broadcast
    await web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [wallet.payer, accKey]
    );

    const acc = await program.account.pda.fetch(accKey.publicKey);
    expect(acc.value).eq(1024);
  });
});
