const web3 = require("@solana/web3.js");
const bs58 = require('bs58');
let secretKey = bs58.decode("3nFYyvUrbPwQAAnW9ZP5Q3T8iKGfTG2");
console.log(`[${web3.Keypair.fromSecretKey(secretKey).secretKey}]`);


privkey = new Uint8Array([139,14,128,46,200,11,57,50,71,208,22,119,239]); // content of id.json here
console.log(bs58.encode(privkey));
