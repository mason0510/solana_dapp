import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { NftMintAndVerify } from "../target/types/nft_mint_and_verify";

describe("nft_mint_and_verify", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NftMintAndVerify as Program<NftMintAndVerify>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
