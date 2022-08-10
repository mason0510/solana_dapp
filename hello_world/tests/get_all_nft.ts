import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { Connection, clusterApiUrl, Keypair, PublicKey } from "@solana/web3.js";

(async () => {
  const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
  const keypair = Keypair.generate();

  const metaplex = new Metaplex(connection);
  metaplex.use(keypairIdentity(keypair));

  const owner = new PublicKey("677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa");
  const allNFTs = await metaplex.nfts().findAllByOwner(owner).run();
  console.log(allNFTs);

})();
