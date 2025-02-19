import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { TransferAccOwnershipInProgram } from "../target/types/transfer_acc_ownership_in_program";

describe("transfer_acc_ownership_in_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferAccOwnershipInProgram as Program<TransferAccOwnershipInProgram>;

  const wallet = provider.wallet as anchor.Wallet;

  it("Is initialized!", async () => {
    console.log(`program id is ${program.programId}`);
    let pda = web3.PublicKey.findProgramAddressSync([], program.programId)[0];

    await program.methods.initialize().rpc();


    let pdaAccInfo = await provider.connection.getAccountInfo(pda);
    console.log(`owner of pda is ${pdaAccInfo.owner}`);

    await program.methods.changeOwner()
      .accounts({
        acc: pda,
      })
      .rpc();

    pdaAccInfo = await provider.connection.getAccountInfo(pda);
    console.log(`owner of pda is ${pdaAccInfo.owner}`);

    // after the ownership has been transferred
    // the account can still be initialized again
    await program.methods.initialize().rpc();
    pdaAccInfo = await provider.connection.getAccountInfo(pda);
    console.log(`owner of pda is ${pdaAccInfo.owner}`);
  });
});
