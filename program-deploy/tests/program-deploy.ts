import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProgramDeploy } from "../target/types/program_deploy";

import fs from 'fs';
// idl中包含program id
let idl = JSON.parse(fs.readFileSync('target/idl/program_deploy.json', 'utf-8'));

describe("program-deploy", () => {

  anchor.setProvider(anchor.AnchorProvider.env());
  let program: Program<ProgramDeploy>;

  before(async () => {
    program = new Program(idl, anchor.getProvider());
  })

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
