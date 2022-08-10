import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplTransfer } from "../target/types/spl_transfer";

describe("spl_transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplTransfer as Program<SplTransfer>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
