import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { DataHolder } from "../target/types/data_holder";
import { DataReader } from "../target/types/data_reader";
import { expect } from "chai";

describe("read_data_from_other_program_account", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const programDataHolder = anchor.workspace.DataHolder as Program<DataHolder>;
  const programDataReader = anchor.workspace.DataReader as Program<DataReader>;

  it("Is initialized!", async () => {
    const pda = web3.PublicKey.findProgramAddressSync([], programDataHolder.programId)[0];

    try {
      await programDataReader.methods.readOtherAccount().accounts({
        otherAccount: pda,
      }).rpc();
    } catch (error) {
      expect(error.error.errorMessage).eq('No data');
    }

    // init pda through data holder
    await programDataHolder.methods.initialize().rpc();
    let pdaAcc = await programDataHolder.account.myData.fetch(pda);
    expect(pdaAcc.x.toNumber()).eq(9);

    // read and deserilize data from the account that other program owns
    await programDataReader.methods.readOtherAccount().accounts({
      otherAccount: pda,
    }).rpc();
  });
});
