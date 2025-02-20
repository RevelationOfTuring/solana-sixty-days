import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Reinit } from "../target/types/reinit";
import { expect } from "chai";

describe("reinit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection = anchor.getProvider().connection;

  const program = anchor.workspace.Reinit as Program<Reinit>;

  it("Is initialized!", async () => {
    const pda = web3.PublicKey.findProgramAddressSync([], program.programId)[0];
    // init
    await program.methods.initialize().rpc();
    let accInfo = await connection.getAccountInfo(pda);
    const orginalBalance = accInfo.lamports;
    console.log(`owner: ${accInfo.owner}, balance: ${accInfo.lamports}`)

    // fail to init again
    try {
      await program.methods.initialize().rpc();
    } catch (error) {
      expect(error.transactionLogs[3]).eq('Allocate: account Address { address: 4r2aubF9KYbk8cCNd8a7rUtpU6qRFoQrbXsZeyhhsuXz, base: None } already in use')
    }

    // 1. transfer all lamports out of pda
    await program.methods.drainAllBalanceFromPda().rpc();
    accInfo = await connection.getAccountInfo(pda);
    // accInfo = null
    expect(accInfo).null;
    // init again successfully
    await program.methods.initialize().rpc();
    accInfo = await connection.getAccountInfo(pda);
    expect(accInfo.owner.toBase58()).eq(program.programId.toBase58());
    console.log(`owner: ${accInfo.owner}, balance: ${accInfo.lamports}`)

    // 2. transfer ownership to system program
    await program.methods.transferOwnershipToSystemProgram().rpc();
    accInfo = await connection.getAccountInfo(pda);
    expect(accInfo.owner.toBase58()).eq(web3.SystemProgram.programId.toBase58());
    expect(accInfo.lamports).eq(orginalBalance);
    // init again successfully
    await program.methods.initialize().rpc();
    accInfo = await connection.getAccountInfo(pda);
    expect(accInfo.owner.toBase58()).eq(program.programId.toBase58());
    console.log(`owner: ${accInfo.owner}, balance: ${accInfo.lamports}`)
  });
});
