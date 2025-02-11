import * as anchor from "@coral-xyz/anchor";
import { Program, BN, AnchorError } from "@coral-xyz/anchor";
import { AssertAndCustomError } from "../target/types/assert_and_custom_error";
import { assert } from "chai";

describe("assert_and_custom_error", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AssertAndCustomError as Program<AssertAndCustomError>;

  it("assert test", async () => {
    // pass
    await program.methods.assert(new BN(50)).rpc();

    // too small
    try {
      await program.methods.assert(new BN(1)).rpc();
    } catch (err) {
      assert.isTrue(err instanceof AnchorError);
      const e = err as AnchorError;
      assert.strictEqual(e.error.errorMessage, 'value is too small');
      assert.strictEqual(e.error.errorCode.code, 'TooSmall');
      assert.strictEqual(e.error.errorCode.number, 6000);
    }

    // too big
    try {
      await program.methods.assert(new BN(101)).rpc();
    } catch (err) {
      assert.isTrue(err instanceof AnchorError);
      const e = err as AnchorError;
      assert.strictEqual(e.error.errorMessage, 'value is too big');
      assert.strictEqual(e.error.errorCode.code, 'TooBig');
      assert.strictEqual(e.error.errorCode.number, 6001);
    }

    try {
      await program.methods.func().rpc();
    } catch (err) {
      assert.isTrue(err instanceof AnchorError);
      const e = err as AnchorError;
      assert.strictEqual(e.error.errorMessage, 'always errors');
      assert.strictEqual(e.error.errorCode.code, 'AlwaysErrors');
      assert.strictEqual(e.error.errorCode.number, 6002);
    }
  });
});
