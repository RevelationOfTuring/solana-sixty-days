import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { InitIfNeeded } from "../target/types/init_if_needed";

describe("init_if_needed", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.InitIfNeeded as Program<InitIfNeeded>;
  const wallet = provider.wallet as anchor.Wallet;
  it("Is initialized!", async () => {
    const pda = web3.PublicKey.findProgramAddressSync([], program.programId)[0];

    // init
    await program.methods.add().rpc();
    let acc = await program.account.data.fetch(pda);
    console.log(acc.data); // 1

    // not init
    await program.methods.add().rpc();
    acc = await program.account.data.fetch(pda);
    console.log(acc.data); // 2

    // not init
    await program.methods.add().rpc();
    acc = await program.account.data.fetch(pda);
    console.log(acc.data); // 3
  });
});
