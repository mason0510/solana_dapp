import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MplTokenTransfer } from "../target/types/mpl_token_transfer";

describe("mpl_token_transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MplTokenTransfer as Program<MplTokenTransfer>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
