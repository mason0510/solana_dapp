import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import NodeWallet from '@project-serum/anchor/dist/cjs/nodewallet';
import { EscrowMarketplace } from '../idl/escrow_marketplace';
//import { AnchorEscrow } from '../target/types/anchor_escrow';
import { PublicKey, SystemProgram, Transaction, Connection, Commitment } from '@solana/web3.js';
import {TOKEN_PROGRAM_ID, createMint, createAccount, getAccount, mintTo} from "@solana/spl-token";
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
  let mint_coin = null;
  let mint_token = null;
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
    //创建nft
    mint_coin = await createMint(provider.connection,
        payer,
        payer.publicKey,
        mintAuthority.publicKey,
        0)
    //创建代币（k币）
    mint_token = await createMint(provider.connection,
        payer,
        payer.publicKey,
        mintAuthority.publicKey,
        0)

    //创建卖家在k币上的ata账户
    seller_coin_account = await createAccount(provider.connection,payer,mint_coin,seller.publicKey);//await mint_coin.createAccount(seller.publicKey);
    //创建买家在k币上的ata账户
    buyer_coin_account = await createAccount(provider.connection,payer,mint_coin,buyer.publicKey) //await mint_coin.createAccount(buyer.publicKey);
    //创建卖家在该NFT上的ata账户
    seller_token_account = await createAccount(provider.connection,payer,mint_token,seller.publicKey) //await mint_token.createAccount(seller.publicKey)
    //创建买家在该NFT上的ata账户
    buyer_token_account = await createAccount(provider.connection,payer,mint_token,buyer.publicKey) //await mint_token.createAccount(buyer.publicKey);

    //初始化创建NFT并给到卖家的nft的ata账户
    await mintTo(
        provider.connection,
        payer,
        mint_token,
        seller_token_account,
        payer.publicKey,
        nft_amount
    );
    //初始化铸币1000个给买家的k币的ata账户
    await mintTo(
        provider.connection,
        payer,
        mint_coin,
        buyer_coin_account,
        payer.publicKey,
        buyer_coin_amount
    );

    let _initializerTokenAccountA = await getAccount(provider.connection,seller_token_account);
    let _takerTokenAccountB = await getAccount(provider.connection,buyer_coin_account);

    assert.ok(Number(_initializerTokenAccountA.amount) as number ==   nft_amount);
    assert.ok(Number(_takerTokenAccountB.amount) == buyer_coin_amount);
  });

  //测试挂单接口
  it("sell nft", async () => {
    const [_vault_account_pda, _vault_account_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("market_vault")),mint_token.toBuffer()],
      program.programId
    );
    vault_account_pda = _vault_account_pda;
    vault_account_bump = _vault_account_bump;

    const [_vault_authority_pda, _vault_authority_bump] = await PublicKey.findProgramAddress(
      [Buffer.from(anchor.utils.bytes.utf8.encode("escrow_owner")),mint_token.toBuffer()],
      program.programId
    );
    vault_authority_pda = _vault_authority_pda;

    await program.rpc.sell(
        vault_authority_pda.publicKey,
      new anchor.BN(buyer_coin_amount),
      {
        accounts: {
          seller: seller.publicKey,
          nftMint: mint_token,
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

    let _vault = await getAccount(provider.connection,vault_account_pda);

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
