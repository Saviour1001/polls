import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GriffyPolls } from "../target/types/griffy_polls";
import { assert } from "chai";

describe("griffy-polls", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GriffyPolls as Program<GriffyPolls>;

  it("Creates a poll", async () => {
    const randomAccount = anchor.web3.Keypair.generate();

    const tx = await program.methods
      .createPoll("What is your favorite color?", ["Red", "Blue"])
      .accounts({
        pollData: randomAccount.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([randomAccount])
      .rpc();

    const pollData = await program.account.pollData.fetch(
      randomAccount.publicKey
    );

    assert.equal(pollData.pollTopic, "What is your favorite color?");
    assert.equal(pollData.pollOptions[0], "Red");
    assert.equal(pollData.pollOptions[1], "Blue");
  });
});
