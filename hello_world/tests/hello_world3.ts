import * as anchor from "@project-serum/anchor";
import { Metaplex, keypairIdentity, bundlrStorage} from "@metaplex-foundation/js";
import { Connection, clusterApiUrl, Keypair,PublicKey } from "@solana/web3.js";

import { Metadata } from "@metaplex-foundation/mpl-token-metadata";


export const MINTS_PROGRAM_ID = new anchor.web3.PublicKey(
    "cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ"
);

describe("load data", () => {


  const randomKeypair = anchor.web3.Keypair.generate();

  const cluster = anchor.web3.clusterApiUrl("devnet");

  const connection = new anchor.web3.Connection(cluster);

  const wallet = new anchor.Wallet(randomKeypair);

  const provider = new anchor.AnchorProvider(connection, wallet, {
    preflightCommitment: "processed",
  });

  it("Fetch Data", async () => {
    try {
      const idl = await anchor.Program.fetchIdl(MINTS_PROGRAM_ID, provider);
      console.log("IDL ", JSON.parse(idl));
      const program = new anchor.Program(idl!, MINTS_PROGRAM_ID, provider);
      //console.log("Accountsssss-- ", program.account.collectionPda.fetch("4qYMXpZWM5SC22iesfxLgJJR4jWPZ93LXojA6geSvJph"));

      const accounts = await program.account.candyMachine.fetch(
          "4qYMXpZWM5SC22iesfxLgJJR4jWPZ93LXojA6geSvJph"
      );

      //console.log("Accounts %o", accounts.data);



      //test2
      //test1
      /*const mint = new PublicKey("4qYMXpZWM5SC22iesfxLgJJR4jWPZ93LXojA6geSvJph");
      const metaplex = Metaplex.make(connection)
          .use(keypairIdentity(randomKeypair))
          .use(bundlrStorage());
      const nft = await metaplex.nfts().findByMint(mint).run();
      console.log("nfts {}",nft);*/


    } catch (error) {
      console.log("Account error ", error);
    }
  });
});