import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";
import { expect } from "chai";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("function arguments", async () => {
    const arr = [new BN(1024), new BN(2048), new BN(4096)];
    const tx = await program.methods.array(arr, new BN(256), 'hello michael.w').rpc();
    console.log("Your transaction signature", tx);
  });

  // set overflow-checks = true
  // it("checks overflow as setting overflow-checks = true", async () => {
  //   // 1+2 pass
  //   await program.methods.f1(new BN(1), new BN(2)).rpc();

  //   // 18446744073709551615 = 2^64-1
  //   try {
  //     await program.methods.f1(new BN('18446744073709551615'), new BN(1)).rpc();
  //   } catch (err) {
  //     expect(err.transactionMessage).eq('Transaction simulation failed: Error processing Instruction 0: Program failed to complete');
  //     expect(err.transactionLogs[2]).eq('Program log: panicked at programs/day2/src/lib.rs:15:17:\nattempt to add with overflow');
  //   }
  // });

  // set overflow-checks = false
  it("checks overflow as setting overflow-checks = false", async () => {
    // 1+2 pass
    await program.methods.f2(new BN(1), new BN(2)).rpc();
    // 18446744073709551615+1 pass
    await program.methods.f1(new BN('18446744073709551615'), new BN(1)).rpc();

    try {
      await program.methods.f2(new BN('18446744073709551615'), new BN(1)).rpc();
    } catch (err) {
      expect(err.transactionMessage).eq('Transaction simulation failed: Error processing Instruction 0: Program failed to complete');
      expect(err.transactionLogs[2]).eq('Program log: panicked at programs/day2/src/lib.rs:20:34:\ncalled `Option::unwrap()` on a `None` value');
    }
  });
});
