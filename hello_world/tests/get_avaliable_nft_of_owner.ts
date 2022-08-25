import { Connection, clusterApiUrl, PublicKey } from '@solana/web3.js';
import bs58 from 'bs58';
//import { Metadata } from "@metaplex-foundation/mpl-token-metadata";
import { CandyMachine } from "@metaplex-foundation/mpl-candy-machine";



const connection = new Connection(clusterApiUrl('devnet'));
const MAX_NAME_LENGTH = 32;
const MAX_URI_LENGTH = 200;
const MAX_SYMBOL_LENGTH = 10;
const MAX_CREATOR_LEN = 32 + 1 + 1;
const MAX_CREATOR_LIMIT = 5;
const MAX_DATA_SIZE = 4 + MAX_NAME_LENGTH + 4 + MAX_SYMBOL_LENGTH + 4 + MAX_URI_LENGTH + 2 + 1 + 4 + MAX_CREATOR_LIMIT * MAX_CREATOR_LEN;
const MAX_METADATA_LEN = 1 + 32 + 32 + MAX_DATA_SIZE + 1 + 1 + 9 + 172;
const CREATOR_ARRAY_START = 1 + 32 + 32 + 4 + MAX_NAME_LENGTH + 4 + MAX_URI_LENGTH + 4 + MAX_SYMBOL_LENGTH + 2 + 1 + 4;

const TOKEN_METADATA_PROGRAM = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');
const CANDY_MACHINE_V2_PROGRAM = new PublicKey('cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ');
const candyMachineId = new PublicKey('4qYMXpZWM5SC22iesfxLgJJR4jWPZ93LXojA6geSvJph');
const MyOwnerId = new PublicKey('677NzkzkDKT9wXDMXGPUvbFp1T7XzJtZZxcRaBAaSvNa');

const getMintAddresses = async (firstCreatorAddress: PublicKey) => {
    console.log("---------",Date.now());
    const metadataAccounts = await connection.getProgramAccounts(
        TOKEN_METADATA_PROGRAM,
        {
            // The mint address is located at byte 33 and lasts for 32 bytes.
            dataSlice: { offset: 33, length: 32 },

            filters: [
                // Only get Metadata accounts.
                { dataSize: MAX_METADATA_LEN },

                // Filter using the first creator.
                {
                    memcmp: {
                        offset: CREATOR_ARRAY_START,
                        bytes: firstCreatorAddress.toBase58(),
                    },
                },
            ],
        },
    );
    console.log("---------",Date.now());

    return metadataAccounts.map((metadataAccountInfo) => (
        bs58.encode(metadataAccountInfo.account.data)
    ));
};

const getCandyMachineCreator = async (candyMachine: PublicKey): Promise<[PublicKey, number]> => (
    PublicKey.findProgramAddress(
        [Buffer.from('candy_machine'), candyMachine.toBuffer()],
        CANDY_MACHINE_V2_PROGRAM,
    )
);

(async () => {

    let collections = [
        /*"5vVzukCYp9eDvqTaW4P9NXNco1E82f8ArXCzEo79NTyo",
        "3YEWaEFXfLtdC4iEA9ge7Pam52hhNbRb3ZTbF27e4CcC",
        "CPugH8xmFxpXM8898T4cRFT9oYHEUoCnEAiseJP2fb49",
        "DZ9bgaLLZUY6H4iqsSRXR4rJ7fs1HhWTMmpY8bNoCMRh",
        "AtmjF2VEffyDov5MZmt6XhQLL9SCr8b3baevKEmQAMXG",
        "3XAZ49Ffke8zkGaiEWYVW1sbhV8ZT4XkQyEdpqTGMbZW",
        "Ctj5KL6y4QHaGFjqYXo9j7PwtpzdBe21rMq984i7Qct7",
        "HBQTomn3ryj2odUcWGAfFehMk9erByVTF2uNSYkqRHuz",
        "HJxHc7Vc4EHWeg1ev3yK8XmXXwqZheXaN7ucQ5xMDqqi",*/
        "HYRw4bjkci823ApqoJj9gJzPCwM563XqtLVwoBND8JLL"
    ];
    for (let i=0; i<collections.length; i++)
    {
        console.log("collection_index %d",i);
        const candyMachineCreator = await getCandyMachineCreator(new PublicKey(collections[i]));
        console.log("candyMachineCreator ",JSON.stringify(candyMachineCreator));
        let nftAddrs = await getMintAddresses(candyMachineCreator[0]);
        for (let i=0; i<nftAddrs.length; i++)
        {
            //get mint
            let mintPubkey = new PublicKey(nftAddrs[i]);
            //let tokenmetaPubkey = await Metadata.getPDA(mintPubkey);
            //const tokenMint = await Metadata.load(connection, tokenmetaPubkey);
            //console.log("tokenMint ",mintPubkey.toBase58())
            //get owner
            const largestAccounts = await connection.getTokenLargestAccounts(
                new PublicKey(mintPubkey.toBase58())
            );
            const largestAccountInfo = await connection.getParsedAccountInfo(
                largestAccounts.value[0].address
            );

            //@ts-ignore
            console.log("tokenMint %s ;owner %s",mintPubkey.toBase58(),largestAccountInfo.value.data.parsed.info.owner);
            //console.log(tokenmeta);
            //console.log("name %s,uri %s",tokenmeta.data.data.name,tokenmeta.data.data.uri);
        }

    }



})();