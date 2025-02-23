import * as anchor from "@coral-xyz/anchor";
import { Program, web3, BN } from "@coral-xyz/anchor";
import { Alice } from "../target/types/alice";
import { Bob } from "../target/types/bob";
import { expect } from "chai";

describe("cpi", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const alice_program = anchor.workspace.Alice as Program<Alice>;
  const bob_program = anchor.workspace.Bob as Program<Bob>;

  it("Is initialized!", async () => {

    const bobAccKey = web3.Keypair.generate();

    // init bob account
    await bob_program.methods.initialize()
      .accounts({
        bobAccount: bobAccKey.publicKey
      })
      .signers([bobAccKey])
      .rpc();

    let data = new BN(1024);
    // set data by bob.set()
    await bob_program.methods.set(data);

    let bobAcc = await bob_program.account.bobData.fetch(bobAccKey.publicKey);
    expect(bobAcc.data.eq(data));

    // set data by alice.call_bob_to_store() -> bob.set()
    data = new BN(2048);
    await alice_program.methods.callBobToStore(data)
      .accounts({
        bobAccount: bobAccKey.publicKey,
        bobProgram: bob_program.programId,
      })
      .rpc();
    expect(bobAcc.data.eq(data));

    // // init alice account
    // const aliceAccKey = web3.Keypair.generate();
    // await alice_program.methods.init()
    // .accounts({
    //   aliceAccount:aliceAccKey.publicKey
    // })
    // .signers([aliceAccKey])
    // .rpc();

    // // pass alice account in
    // console.log(`alice program: ${alice_program.programId}`)
    // console.log(`bob program: ${bob_program.programId}`)
    // await alice_program.methods.callBobToStore(new BN(100))
    // .accounts({
    //   bobAccount:aliceAccKey.publicKey,
    //   bobProgram:bob_program.programId
    // })
    // .rpc();
  });
});
