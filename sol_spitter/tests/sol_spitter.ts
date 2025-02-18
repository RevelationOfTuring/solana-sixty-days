import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { SolSpitter } from "../target/types/sol_spitter";

describe("sol_spitter", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolSpitter as Program<SolSpitter>;

  it("Transfer sol", async () => {

    const recipient = anchor.web3.Keypair.generate();
    await printAccountBalance(recipient.publicKey);

    // transfer 1 sol
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    await program.methods.transferSol(amount)
      .accounts({
        signer: provider.wallet.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();
    await printAccountBalance(recipient.publicKey);
  });

  async function printAccountBalance(account) {
    const balance = await anchor.getProvider().connection.getBalance(account);
    console.log(`${account} has ${balance / anchor.web3.LAMPORTS_PER_SOL} SOL`);
  }

  it('Split sol', async () => {
    const recipient1 = web3.Keypair.generate();
    const recipient2 = web3.Keypair.generate();
    const recipient3 = web3.Keypair.generate();
    const recipient4 = web3.Keypair.generate();

    const accountMeta1 = { pubkey: recipient1.publicKey, isWritable: true, isSigner: false };
    const accountMeta2 = { pubkey: recipient2.publicKey, isWritable: true, isSigner: false };
    const accountMeta3 = { pubkey: recipient3.publicKey, isWritable: true, isSigner: false };
    const accountMeta4 = { pubkey: recipient4.publicKey, isWritable: true, isSigner: false };


    await program.methods.splitSol(new BN(4 * web3.LAMPORTS_PER_SOL))
      .remainingAccounts([accountMeta1, accountMeta2, accountMeta3, accountMeta4]).rpc();

    await printAccountBalance(recipient1.publicKey);
    await printAccountBalance(recipient2.publicKey);
    await printAccountBalance(recipient3.publicKey);
    await printAccountBalance(recipient4.publicKey);
  })
});
