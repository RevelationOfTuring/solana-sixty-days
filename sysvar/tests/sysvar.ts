import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Sysvar } from "../target/types/sysvar";

describe("sysvar", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sysvar as Program<Sysvar>;

  it("Get day of week", async () => {
    await program.methods.getDayOfTheWeek().rpc();
  });

  it("Get recent blockhashes", async () => {
    await program.methods.getLatestBlockhashes()
      .accounts({
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
      })
      .rpc();
  });

  it("Get epoch schedule", async () => {
    await program.methods.getEpochSchedule().rpc();
  });

  it("Get rent", async () => {
    await program.methods.getRent().rpc();
  });

  it("Get instruction", async () => {
    await program.methods.getInstruction(1024)
      .accounts({
        stakeHistory: anchor.web3.SYSVAR_STAKE_HISTORY_PUBKEY,
        recentBlockhashes: anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
        instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .rpc();
  });

  it("Get LastRestartSlot", async () => {
    await program.methods.getLastRestartSlot()
      .accounts({
        // the pubkey of LastRestartSlot sysvar isn't defined in anchor.web3
        lastRestartSlot: new anchor.web3.PublicKey('SysvarLastRestartS1ot1111111111111111111111')
      })
      .rpc();
  });
});
