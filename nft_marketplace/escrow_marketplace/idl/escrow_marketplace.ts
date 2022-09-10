export type EscrowMarketplace = {
  "version": "0.1.0",
  "name": "escrow_marketplace",
  "instructions": [
    {
      "name": "sell",
      "accounts": [
        {
          "name": "seller",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "vaultAuthorityKey",
          "type": "publicKey"
        },
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancel",
      "accounts": [
        {
          "name": "seller",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "buy",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "buyerCoinAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "kCoinMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftTokenMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerCoinAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "sellOrder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "publicKey"
          },
          {
            "name": "mintAccount",
            "type": "publicKey"
          },
          {
            "name": "nftTokenAccount",
            "type": "publicKey"
          },
          {
            "name": "price",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "SupportCoin",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "KCoin",
            "fields": [
              "publicKey"
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InSufficientFunds",
      "msg": "InSufficientFunds"
    },
    {
      "code": 6001,
      "name": "NotSupportCoin",
      "msg": "NotSupportCoin"
    },
    {
      "code": 6002,
      "name": "NftNotMatched",
      "msg": "NftNotMatched"
    },
    {
      "code": 6003,
      "name": "SellerNotMatched",
      "msg": "SellerNotMatched"
    },
    {
      "code": 6004,
      "name": "UnknownError",
      "msg": "UnknownError"
    }
  ]
};

export const IDL: EscrowMarketplace = {
  "version": "0.1.0",
  "name": "escrow_marketplace",
  "instructions": [
    {
      "name": "sell",
      "accounts": [
        {
          "name": "seller",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "nftMint",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "vaultAuthorityKey",
          "type": "publicKey"
        },
        {
          "name": "price",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancel",
      "accounts": [
        {
          "name": "seller",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "buy",
      "accounts": [
        {
          "name": "buyer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "buyerCoinAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "kCoinMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "nftTokenMintAccount",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "buyerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerCoinAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "sellerTokenAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "seller",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAccount",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vaultAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "rent",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "sellOrder",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "wallet",
            "type": "publicKey"
          },
          {
            "name": "mintAccount",
            "type": "publicKey"
          },
          {
            "name": "nftTokenAccount",
            "type": "publicKey"
          },
          {
            "name": "price",
            "type": "u64"
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "SupportCoin",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "KCoin",
            "fields": [
              "publicKey"
            ]
          }
        ]
      }
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InSufficientFunds",
      "msg": "InSufficientFunds"
    },
    {
      "code": 6001,
      "name": "NotSupportCoin",
      "msg": "NotSupportCoin"
    },
    {
      "code": 6002,
      "name": "NftNotMatched",
      "msg": "NftNotMatched"
    },
    {
      "code": 6003,
      "name": "SellerNotMatched",
      "msg": "SellerNotMatched"
    },
    {
      "code": 6004,
      "name": "UnknownError",
      "msg": "UnknownError"
    }
  ]
};
