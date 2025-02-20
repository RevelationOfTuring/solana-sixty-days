import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { TransferSolOutOfPda } from "../target/types/transfer_sol_out_of_pda";

describe("transfer_sol_out_of_pda", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferSolOutOfPda as Program<TransferSolOutOfPda>;
  const connection = provider.connection;
  it("Is initialized!", async () => {
    console.log(`signer balance: ${await connection.getBalance(provider.wallet.publicKey)}`);

    const pda = web3.PublicKey.findProgramAddressSync([], program.programId)[0];
    console.log(`pda balance: ${await connection.getBalance(pda)}`);

    // init pda
    await program.methods.initialize().rpc();
    console.log(`pda balance: ${await connection.getBalance(pda)}`);

    // transfer 2 sol into the pda
    await program.methods.donate(new BN(2 * web3.LAMPORTS_PER_SOL)).rpc();
    console.log(`pda balance: ${await connection.getBalance(pda)}`);

    // transfer 2 sol out of pda
    const recipient = new web3.Keypair();
    await program.methods.withdraw(new BN(2 * web3.LAMPORTS_PER_SOL)).accounts({
      recipent: recipient.publicKey,
    }).rpc();
    console.log(`pda balance: ${await connection.getBalance(pda)}`);
    console.log(`recipent balance: ${await connection.getBalance(recipient.publicKey)}`);
  });
});
