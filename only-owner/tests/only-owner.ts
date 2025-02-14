import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { OnlyOwner } from "../target/types/only_owner";
import { expect } from "chai";

describe("only-owner", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OnlyOwner as Program<OnlyOwner>;

  it("Pass", async () => {
    await program.methods.onlyOwner().rpc();
  });

  it("Fail", async () => {
    const randomKey = new anchor.web3.Keypair();
    try {
      await program.methods.onlyOwner()
        .accounts({
          signer: randomKey.publicKey,
        })
        .signers([randomKey])
        .rpc()
    } catch (err) {
      expect(err.logs[2]).eq('Program log: AnchorError thrown in programs/only-owner/src/lib.rs:19. Error Code: NotOwner. Error Number: 6000. Error Message: Only owner can call!.');
    }
  });
});
