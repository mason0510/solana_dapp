pub const K_COIN:  &'static str = "5d1i4wKHhGXXkdZB22iKD1SqU6pkBeTCwFEMqo7xy39h";
pub const VAULT_SIGNER: &[u8] = b"escrow_owner";
pub const VAULT_PREFIX: &[u8] = b"market_vault";
//pub const ESCROW: &[u8] = b"market_escrow";
pub const ORDER_SPACE: usize = 8 +      // anchor discriminator
    32 +                                // seller wallet
    32 +                                // seller mint token account
    32 +                                // seller token account of nft
    32 +                                    // seller token account of K coin
    8                                   // price
;