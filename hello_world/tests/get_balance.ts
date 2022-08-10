import { Connection, PublicKey } from "@solana/web3.js";

// connection
const connection = new Connection("https://api.devnet.solana.com");

const tokenAccount1Pubkey = new PublicKey("9hUYW9s2c98GfjZb6JvW62BYEt3ryxGmeMBkhgSqmZtW");

const tokenAccount2Pubkey = new PublicKey("CFEPU5Jd6DNj8gpjPLJ1d9i4xSJDGYNV7n6qw53zE3n1");

(async () => {
  let tokenAccountBalance = await connection.getTokenAccountBalance(tokenAccount1Pubkey);
  console.log(`decimals: ${tokenAccountBalance.value.decimals}, amount: ${tokenAccountBalance.value.amount}`);
})();
