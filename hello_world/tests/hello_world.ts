import { Connection, clusterApiUrl, PublicKey } from '@solana/web3.js';
import bs58 from 'bs58';
import { Metadata } from "@metaplex-foundation/mpl-token-metadata";

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
    //const candyMachineCreator = await getCandyMachineCreator(candyMachineId);
    //let addressArr = await getMintAddresses(candyMachineCreator[0]);
    //console.log("addressArr ",addressArr);
    let addressArr =  [
             'HMwfWAuf9mU3fqKAUBdtRvYQR2GmyUrYVRyWQgMvt6qu',
            'J5oq1hDvs451bFauLnVb6pSmgk8ibWci7kW6MUKznwfp',
            'EGb7fJQLFb85JSyjWNuzQiw454SKh22HXc75je4aZ6yL',
            '2g2TbvUirD3SjGtG7wVvJbR6P5rkpNkS5B3NWec898ey',
            'AnUuYXHPdXKRGT8M5xCdrXpgpookByueSr3SmupuSnx2',
            'FaMqoVd5GRRbYqsxaKLHMLEU7ueYJDS6vX77qxopGTbj',
            'A6PbqsuL3LMNRS4QohBPjYvNxqE74joT875yh8cT2S3U',
            'CFYbnVZYC4GivFXJdeqLYCPQCTGbTaNm4h7gbW2xX7ye',
            '2Pgzah7TQi73maLNr9fYcPzi7PC3xuUGfbvN5bjs8EKw',
            'DAejTZMZYmm8b6qrAAnACWbXycSrkunDzDjURdNzQwdo',
            'FSUXYdodPrKZJiXRHHXhjFC5huPpFLcJ1gb8DAaqt6j4',
            '4Wzkg4zXV7LV56rWBNseAMjTPcgfMT5tgcvpp9oyGmuY',
            '5XsheKfoM8egoGtCNegBvPTiBGmUmWJPyK4wb9rKLHT6',
            'CTgkFKfKVHLgp5VfKYEC8sim39uj7rdHAAecFn3Fe3JL',
            'FX7obtWFUT2zWv8URD7XBiGnWYXeLrTjZdKWEi6xzkNK',
            '9y3m3xHnptuifMcNaErc8BzadzUHRmF31gQz1g8EZyyq',
            '3sLiFK3gGXEfXwrjyfUCv2VGdjdfFk6aW82Y6AVUPSaW',
            '2WuKR7piZRv2Xk9kufTjKcFq7jMLspKGb7P5LrPRhiYq',
            '7QFd3dov2TvoMtnh8stHmJxdUCCv2EUPbwEE16dGb1PB',
            'C6mn3KtYsvzFARHAuekfucM7FKSGFipmcotAShFoG7U2',
            '7jzivdBGZoMEPwdYNGBT3wBUjg8312tQwHBaoATx5QHH',
            'YJiSD3yEsshJ96qKcThssnVQWfo4FR1ocYDgaC4nWsv',
            '3MYPJRsdUDSrMQjt1xhmrLwANSPRaCXniewxtwkYrgyQ',
            'FQEEizWgZW9dhvWNKBwrRFXRC6MyaQniXKPo9mrbrUL2',
            'DQU96GWVYW9T1uzi1EesbsB7Eyu2ph3KKV1uvMgZeeeh',
            '73AJUGcFCz5azHf5EEPGfw6PGek19tcnzJqK6hhwDUBp',
            'friHEsDxft3KPVzQpWFPuNx1wcBuxGouWCoFE23ebBe',
            'CtFjFmsXEJhFvJ2LEDFwKca9H8QJV5YXYij5FctUMBEt',
            '3YyzV6Pwb5Pewh1toBFrpshgrCQ8LZ8yRjKzoaEC9rq2',
            'FHwqdkB3iWD3ZMJc1n7VWAXct48w6E9wcpBDkQepfFBj'
        ];
    for (let i=0; i<20; i++)
    {
        let mintPubkey = new PublicKey(addressArr[i]);
        let tokenmetaPubkey = await Metadata.getPDA(mintPubkey);
        const tokenmeta = await Metadata.load(connection, tokenmetaPubkey);
        //console.log(tokenmeta);
        console.log("name %s,uri %s",tokenmeta.data.data.name,tokenmeta.data.data.uri);
    }

})();