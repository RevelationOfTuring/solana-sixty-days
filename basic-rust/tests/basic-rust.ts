import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { BasicRust } from "../target/types/basic_rust";

describe("basic-rust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BasicRust as Program<BasicRust>;

  it("Is initialized!", async () => {
    const v = [new BN(1), new BN(1092), new BN(3), new BN(2), new BN(5), new BN(14)];
    await program.methods.func(v).rpc();
  });
});
