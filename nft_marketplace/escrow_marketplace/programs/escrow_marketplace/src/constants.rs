pub const K_COIN:  &'static str = "5d1i4wKHhGXXkdZB22iKD1SqU6pkBeTCwFEMqo7xy39h";
pub const ESCROW_PDA_SEED: &[u8] = b"escrow";
//todo:在anchor里初始化order
pub const ORDER_SPACE: usize = 8 +      // anchor discriminator
    32 +                                // seller wallet
    32 +                                // seller mint token account
    32 +                                // seller token account
    8                                   // price

;

/***
 pub initializer_key: Pubkey,
    pub seller_mint_token_account: Pubkey,
    pub seller_token_account: Pubkey,
    pub price: u64,
*/