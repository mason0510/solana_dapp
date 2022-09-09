import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { EscrowMarketplace } from '../target/types/escrow_marketplace';
//import { AnchorEscrow } from '../target/types/anchor_escrow';
import { PublicKey, SystemProgram, Transaction, Connection, Commitment } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";
import { assert } from "chai";

describe('escrow_marketplace', () => {
  const commitment: Commitment = 'processed';
  const connection = new Connection('https://api.devnet.solana.com', { commitment, wsEndpoint: 'wss://api.devnet.solana.com/' });
  const options = anchor.Provider.defaultOptions();
  const wallet = NodeWallet.local();
  const provider = new anchor.Provider(connection, wallet, options);

  anchor.setProvider(provider);

  const program = anchor.workspace.EscrowMarketplace as Program<EscrowMarketplace>;

  //mint A为coin
  //mint B为nft
  let mint_coin = null as Token;


  let mint_token = null as Token;
  let seller_coin_account = null;
  let seller_token_account = null;
  let buyer_coin_account = null;
  let buyer_token_account = null;
  let vault_account_pda = null;
  let vault_account_bump = null;
  let vault_authority_pda = null;

  const buyer_coin_amount = 1000;
  const nft_amount = 1;

  const escrowAccount = anchor.web3.Keypair.generate();
  const payer = anchor.web3.Keypair.generate();
  const mintAuthority = anchor.web3.Keypair.generate();
  const seller = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();

  it("Initialize program state", async () => {
    // Airdropping tokens to a payer.
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(payer.publicKey, 1000000000),
      "processed"
    );

    // Fund Main Accounts
    await provider.send(
      (() => {
        const tx = new Transaction();
        tx.add(
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: seller.publicKey,
            lamports: 100000000,
          }),
          SystemProgram.transfer({
            fromPubkey: payer.publicKey,
            toPubkey: buyer.publicKey,
            lamports: 100000000,
          })
        );
        return tx;
      })(),
      [payer]
    );

    mint_coin = await Token.createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );

    mint_token = await Token.createMint(
      provider.connection,
      payer,
      mintAuthority.publicKey,
      null,
      0,
      TOKEN_PROGRAM_ID
    );
    console.log("000A");
    seller_coin_account = await mint_coin.createAccount(seller.publicKey);
    buyer_coin_account = await mint_coin.createAccount(buyer.publicKey);

    seller_token_account = await mint_token.createAccount(seller.publicKey);
    buyer_token_account = await mint_token.createAccount(buyer.publicKey);
  console.log("0000");
    await mint_token.mintTo(
        seller_token_account,
      mintAuthority.publicKey,
      [mintAuthority],
      nft_amount
    );

    await mint_coin.mintTo(
        buyer_coin_account,
      mintAuthority.publicKey,
      [mintAuthority],
      buyer_coin_amount
    );
    console.log("0001");
    let _initializerTokenAccountA = await mint_token.getAccountInfo(seller_token_account);
    let _takerTokenAccountB = await mint_coin.getAccountInfo(buyer_coin_account);

    console.log("init_state_0002_",JSON.stringify(_initializerTokenAccountA));
    console.log("init_state_0003_",JSON.stringify(_takerTokenAccountB));



    assert.ok(_initializerTokenAccountA.amount.toNumber() == nft_amount);
    assert.ok(_takerTokenAccountB.amount.toNumber() == buyer_coin_amount);
  });

  it("sell nft", async () => {
    const [_vault_account_pda, _vault_account_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("market_vault"))],
      program.programId
    );
    vault_account_pda = _vault_account_pda;
    vault_account_bump = _vault_account_bump;

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("escrow_owner"))],
      program.programId
    );
    vault_authority_pda = _vault_authority_pda;

    console.log("0003__",escrowAccount.publicKey.toBase58());

    /***
     *
     *  seller: payer.pubkey(),
     *             nft_mint: nft_mint_key,
     *             vault_account: vault_account_pda,
     *             seller_token_account: seller_token_account,
     *             escrow_account: escrow_account.pubkey(),
     *             system_program: Pubkey::from_str(SYSTEM_PROGRAM_ID).unwrap(),
     *             rent: Pubkey::from_str(SYSTEM_RENT_ID).unwrap(),
     *             token_program: Pubkey::from_str(SPL_PROGRAM_ID).unwrap(),
     * */
    await program.rpc.sell(
        vault_authority_pda.publicKey,
      new anchor.BN(buyer_coin_amount),
      {
        accounts: {
          seller: seller.publicKey,
          nftMint: mint_token.publicKey,
          vaultAccount: vault_account_pda,
          sellerTokenAccount: seller_token_account,
          escrowAccount: escrowAccount.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        signers: [escrowAccount, seller],
      }
    );

    let _vault = await mint_token.getAccountInfo(vault_account_pda);

    let _escrowAccount = await program.account.sellOrder.fetch(
      escrowAccount.publicKey
    );

    // Check that the new owner is the PDA.
    assert.ok(_vault.owner.equals(vault_authority_pda));

    // Check that the values in the escrow account match what we expect.
    console.log("0001__",escrowAccount.publicKey);
    assert.ok(_escrowAccount.wallet.equals(seller.publicKey));
    assert.ok(_escrowAccount.price.toNumber() == buyer_coin_amount);
    assert.ok(
      _escrowAccount.nftTokenAccount.equals(seller_token_account)
    );

  });
/*
  it("buy nft", async () => {
    await program.rpc.buy({
      accounts: {
        buyer: buyer.publicKey,
        buyerCoinAccount: buyer_coin_account,
        buyerTokenAccount: buyer_token_account,
        sellerCoinAccount: seller_coin_account,
        sellerTokenAccount: seller_token_account,
        seller: seller.publicKey,
        escrowAccount: escrowAccount.publicKey,
        vaultAccount: vault_account_pda,
        vaultAuthority: vault_authority_pda,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [buyer]
    });

    let _buyer_coin_account = await mint_coin.getAccountInfo(buyer_coin_account);
    let _buyer_token_account = await mint_token.getAccountInfo(buyer_token_account);
    let _seller_coin_account = await mint_coin.getAccountInfo(seller_coin_account);
    let _seller_token_account = await mint_token.getAccountInfo(seller_token_account);

    assert.ok(_buyer_coin_account.amount.toNumber() == 0);
    assert.ok(_seller_coin_account.amount.toNumber() == buyer_coin_amount);
    assert.ok(_seller_token_account.amount.toNumber() == 0);
    assert.ok(_buyer_token_account.amount.toNumber() == nft_amount);
  });

  it("sell nft and then cancel", async () => {
    // Put back tokens into initializer token A account.
    await mint_token.mintTo(
      seller_token_account,
      mintAuthority.publicKey,
      [mintAuthority],
      nft_amount
    );

    await program.rpc.initialize(
        vault_account_bump,
        new anchor.BN(buyer_coin_amount),
      {
        accounts: {
          initializer: seller.publicKey,
          nftMint: mint_token.publicKey,
          vaultAccount: vault_account_pda,
          sellerTokenAccount: seller_token_account,
          escrowAccount: escrowAccount.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
          tokenProgram: TOKEN_PROGRAM_ID,
        },
        instructions: [
          await program.account.escrowAccount.createInstruction(escrowAccount),
        ],
        signers: [escrowAccount, seller],
      }
    );

    await program.rpc.cancel({
      accounts: {
        initializer: seller.publicKey,
        sellerTokenAccount: seller_token_account,
        vaultAccount: vault_account_pda,
        vaultAuthority: vault_authority_pda,
        escrowAccount: escrowAccount.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      },
      signers: [seller]
    });

    // Check the final owner should be the provider public key.
    const _initializerTokenAccountA = await mint_token.getAccountInfo(seller_token_account);
    assert.ok(_initializerTokenAccountA.owner.equals(seller.publicKey));

    // Check all the funds are still there.
    assert.ok(_initializerTokenAccountA.amount.toNumber() == nft_amount);
  });*/
});
