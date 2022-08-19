import {Metaplex, keypairIdentity, toPublicKey} from "@metaplex-foundation/js";
import {
    Connection,
    clusterApiUrl,
    Keypair,
    LAMPORTS_PER_SOL, PublicKey,
} from "@solana/web3.js";
// @ts-ignore
import dotenv from "dotenv";
import * as bip39 from "bip39";
import {VerifyNftCollectionInput} from "@metaplex-foundation/js/src/plugins/nftModule/operations";


dotenv.config();

(async () => {
    const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
/*    const keypair = Keypair.fromSecretKey(
        Buffer.from(JSON.parse(process.env.SOLANA_KEYPAIR!.toString()))
    );*/
    //
  /*  const mnemonic = bip39.generateMnemonic();
    const seed = bip39.mnemonicToSeedSync(mnemonic, ""); // (mnemonic, password)
    const keypair = Keypair.fromSeed(seed.slice(0, 32));*/
    //first_deploy
    const keypair = Keypair.fromSecretKey(
        //deploy_first
        //Uint8Array.from([139,14,128,46,200,11,57,50,71,208,220,24,10,149,79,119,238,162,131,5,65,56,185,184,33,58,219,86,135,60,18,219,75,219,33,236,116,80,231,126,73,97,203,85,180,22,51,182,246,70,179,70,134,110,172,68,15,199,115,243,248,108,119,239])
        //wallet/2.json
        Uint8Array.from([110,132,84,43,151,192,22,179,203,21,31,33,16,102,195,187,48,183,220,203,134,26,84,35,206,225,92,198,191,237,156,27,73,83,174,78,134,253,220,104,174,173,251,168,154,202,79,47,211,2,200,202,57,43,146,64,123,186,77,160,157,228,46,212])
    );
    console.log("prikey %s pubkey ",keypair.publicKey.toBase58())

    const metaplex = new Metaplex(connection);
    metaplex.use(keypairIdentity(keypair));

    const feePayerAirdropSignature = await connection.requestAirdrop(
        keypair.publicKey,
        LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(feePayerAirdropSignature);

    const mintNFTResponse = await metaplex.nfts().create({
        name: "test1",
        collection: toPublicKey("6P64iPbit6iUbwMj55pXXEu7GxUaE9jPVqWCmomyqPph"),
        uri: "https://ffaaqinzhkt4ukhbohixfliubnvpjgyedi3f2iccrq4efh3s.arweave.net/KUAIIbk6p8oo4XHRcq0U__C2r0mwQaNl0gQow4Qp9yk",
        maxSupply: 1,
        sellerFeeBasisPoints: 500
    }).run();

    console.log(JSON.stringify(mintNFTResponse));

    //metaplex.nfts().verifyCollection(VerifyNftCollectionInput{

   // })//approveCollectionAuthority()  SetAndVerifyCollection
})();
