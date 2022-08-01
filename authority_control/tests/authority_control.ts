import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { AuthorityControl } from "../target/types/authority_control";

describe("authority_control", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AuthorityControl as Program<AuthorityControl>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
