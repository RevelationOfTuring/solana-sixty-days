import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Close } from "../target/types/close";
import { expect } from "chai";

describe("close", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Close as Program<Close>;

  it("Is initialized!", async () => {
    const pda = web3.PublicKey.findProgramAddressSync([], program.programId)[0];
    let acc = await program.account.pda.fetchNullable(pda);
    expect(acc).null;

    await program.methods.initialize().rpc();
    acc = await program.account.pda.fetch(pda);
    expect(acc.x).eq(0);

    await program.methods.close().accounts({
      pda,
    }).rpc();
    acc = await program.account.pda.fetchNullable(pda);
    expect(acc).null;
  });
});
