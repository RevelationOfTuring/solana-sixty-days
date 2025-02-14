import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Event } from "../target/types/event";

describe("event", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Event as Program<Event>;

  it("Emit event!", async () => {
    const listenEvent1 = program.addEventListener('myEvent1', (event, slot) => {
      console.log(`slot ${slot} event i ${event.i}`)
    });

    const listenEvent2 = program.addEventListener('myEvent2', (event, slot) => {
      console.log(`slot ${slot} event.i ${event.i} event.msg ${event.msg}`)
    });


    await program.methods.initialize('michael.w').rpc();

    // sleep for 2s to ensure the listener has time to catch the event
    await new Promise((resolve) => setTimeout(resolve, 2000));

    program.removeEventListener(listenEvent1);
    program.removeEventListener(listenEvent2);
  });
});
