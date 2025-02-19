import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { TransferSolToPda } from "../target/types/transfer_sol_to_pda";

describe("transfer_sol_to_pda", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferSolToPda as Program<TransferSolToPda>;
  const wallet = provider.wallet as anchor.Wallet;
  it("Is initialized!", async () => {

    // get a pda
    const pda = web3.PublicKey.findProgramAddressSync([], program.programId)[0];

    let balance = await provider.connection.getBalance(pda);
    console.log(balance);

    const tx = new web3.Transaction().add(
      web3.SystemProgram.transfer({
        fromPubkey: wallet.publicKey,
        toPubkey: pda,
        lamports: web3.LAMPORTS_PER_SOL,
      })
    );

    await web3.sendAndConfirmTransaction(
      provider.connection,
      tx,
      [wallet.payer]
    );

    balance = await provider.connection.getBalance(pda);
    console.log(balance);
  });
});
