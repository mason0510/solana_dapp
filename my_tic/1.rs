#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
use anchor_lang::prelude::*;
use instructions::*;
use state::game::Tile;
pub mod errors {
    use anchor_lang::error_code;
    #[repr(u32)]
    pub enum TicTacToeError {
        TileOutOfBounds,
        TileAlreadySet,
        GameAlreadyOver,
        NotPlayersTurn,
        GameAlreadyStarted,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TicTacToeError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&TicTacToeError::TileOutOfBounds,) => {
                    ::core::fmt::Formatter::write_str(f, "TileOutOfBounds")
                }
                (&TicTacToeError::TileAlreadySet,) => {
                    ::core::fmt::Formatter::write_str(f, "TileAlreadySet")
                }
                (&TicTacToeError::GameAlreadyOver,) => {
                    ::core::fmt::Formatter::write_str(f, "GameAlreadyOver")
                }
                (&TicTacToeError::NotPlayersTurn,) => {
                    ::core::fmt::Formatter::write_str(f, "NotPlayersTurn")
                }
                (&TicTacToeError::GameAlreadyStarted,) => {
                    ::core::fmt::Formatter::write_str(f, "GameAlreadyStarted")
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TicTacToeError {
        #[inline]
        fn clone(&self) -> TicTacToeError {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for TicTacToeError {}
    impl TicTacToeError {
        /// Gets the name of this [#enum_name].
        pub fn name(&self) -> String {
            match self {
                TicTacToeError::TileOutOfBounds => "TileOutOfBounds".to_string(),
                TicTacToeError::TileAlreadySet => "TileAlreadySet".to_string(),
                TicTacToeError::GameAlreadyOver => "GameAlreadyOver".to_string(),
                TicTacToeError::NotPlayersTurn => "NotPlayersTurn".to_string(),
                TicTacToeError::GameAlreadyStarted => "GameAlreadyStarted".to_string(),
            }
        }
    }
    impl From<TicTacToeError> for u32 {
        fn from(e: TicTacToeError) -> u32 {
            e as u32 + anchor_lang::error::ERROR_CODE_OFFSET
        }
    }
    impl From<TicTacToeError> for anchor_lang::error::Error {
        fn from(error_code: TicTacToeError) -> anchor_lang::error::Error {
            anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                error_name: error_code.name(),
                error_code_number: error_code.into(),
                error_msg: error_code.to_string(),
                error_origin: None,
                compared_values: None,
            })
        }
    }
    impl std::fmt::Display for TicTacToeError {
        fn fmt(
            &self,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::result::Result<(), std::fmt::Error> {
            match self {
                TicTacToeError::TileOutOfBounds => <Self as std::fmt::Debug>::fmt(self, fmt),
                TicTacToeError::TileAlreadySet => <Self as std::fmt::Debug>::fmt(self, fmt),
                TicTacToeError::GameAlreadyOver => <Self as std::fmt::Debug>::fmt(self, fmt),
                TicTacToeError::NotPlayersTurn => <Self as std::fmt::Debug>::fmt(self, fmt),
                TicTacToeError::GameAlreadyStarted => <Self as std::fmt::Debug>::fmt(self, fmt),
            }
        }
    }
}
pub mod instructions {
    pub use play::*;
    pub use setup_game::*;
    pub mod play {
        use crate::errors::TicTacToeError;
        use crate::state::game::*;
        use anchor_lang::prelude::*;
        pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
            let game = &mut ctx.accounts.game;
            if game.current_player() != ctx.accounts.player.key() {
                return Err(
                    anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                        error_name: TicTacToeError::NotPlayersTurn.name(),
                        error_code_number: TicTacToeError::NotPlayersTurn.into(),
                        error_msg: TicTacToeError::NotPlayersTurn.to_string(),
                        error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                            anchor_lang::error::Source {
                                filename: "programs/tic-tac-toe/src/instructions/play.rs",
                                line: 8u32,
                            },
                        )),
                        compared_values: None,
                    })
                    .with_pubkeys((game.current_player(), ctx.accounts.player.key())),
                );
            };
            game.play(&tile)
        }
        pub struct Play<'info> {
            #[account(mut)]
            pub game: Account<'info, Game>,
            pub player: Signer<'info>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for Play<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                let game: anchor_lang::accounts::account::Account<Game> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("game"))?;
                let player: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("player"))?;
                if !game.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("game"));
                }
                Ok(Play { game, player })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for Play<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.game.to_account_infos());
                account_infos.extend(self.player.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for Play<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.game.to_account_metas(None));
                account_metas.extend(self.player.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for Play<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.game, program_id)
                    .map_err(|e| e.with_account_name("game"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_play {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`Play`].
            pub struct Play {
                pub game: anchor_lang::solana_program::pubkey::Pubkey,
                pub player: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for Play
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.game, writer)?;
                    borsh::BorshSerialize::serialize(&self.player, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for Play {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.game, false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.player,
                            true,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_play {
            use super::*;
            /// Generated CPI struct of the accounts for [`Play`].
            pub struct Play<'info> {
                pub game: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub player: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for Play<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.game),
                        false,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.player),
                            true,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for Play<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.game));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.player));
                    account_infos
                }
            }
        }
    }
    pub mod setup_game {
        use crate::state::game::*;
        use anchor_lang::prelude::*;
        pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
            ctx.accounts
                .game
                .start([ctx.accounts.player_one.key(), player_two])
        }
        pub struct SetupGame<'info> {
            # [account (init , payer = player_one , space = Game :: MAXIMUM_SIZE + 8)]
            pub game: Account<'info, Game>,
            #[account(mut)]
            pub player_one: Signer<'info>,
            pub system_program: Program<'info, System>,
        }
        #[automatically_derived]
        impl<'info> anchor_lang::Accounts<'info> for SetupGame<'info>
        where
            'info: 'info,
        {
            #[inline(never)]
            fn try_accounts(
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
                accounts: &mut &[anchor_lang::solana_program::account_info::AccountInfo<'info>],
                ix_data: &[u8],
                __bumps: &mut std::collections::BTreeMap<String, u8>,
            ) -> anchor_lang::Result<Self> {
                if accounts.is_empty() {
                    return Err(anchor_lang::error::ErrorCode::AccountNotEnoughKeys.into());
                }
                let game = &accounts[0];
                *accounts = &accounts[1..];
                let player_one: Signer =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("player_one"))?;
                let system_program: anchor_lang::accounts::program::Program<System> =
                    anchor_lang::Accounts::try_accounts(program_id, accounts, ix_data, __bumps)
                        .map_err(|e| e.with_account_name("system_program"))?;
                let __anchor_rent = Rent::get()?;
                let game = {
                    let actual_field = game.to_account_info();
                    let actual_owner = actual_field.owner;
                    let space = Game::MAXIMUM_SIZE + 8;
                    let pa: anchor_lang::accounts::account::Account<Game> = if !false
                        || actual_owner == &anchor_lang::solana_program::system_program::ID
                    {
                        let payer = player_one.to_account_info();
                        let __current_lamports = game.lamports();
                        if __current_lamports == 0 {
                            let lamports = __anchor_rent.minimum_balance(space);
                            let cpi_accounts = anchor_lang::system_program::CreateAccount {
                                from: payer.to_account_info(),
                                to: game.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::create_account(
                                cpi_context.with_signer(&[]),
                                lamports,
                                space as u64,
                                program_id,
                            )?;
                        } else {
                            let required_lamports = __anchor_rent
                                .minimum_balance(space)
                                .max(1)
                                .saturating_sub(__current_lamports);
                            if required_lamports > 0 {
                                let cpi_accounts = anchor_lang::system_program::Transfer {
                                    from: payer.to_account_info(),
                                    to: game.to_account_info(),
                                };
                                let cpi_context = anchor_lang::context::CpiContext::new(
                                    system_program.to_account_info(),
                                    cpi_accounts,
                                );
                                anchor_lang::system_program::transfer(
                                    cpi_context,
                                    required_lamports,
                                )?;
                            }
                            let cpi_accounts = anchor_lang::system_program::Allocate {
                                account_to_allocate: game.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::allocate(
                                cpi_context.with_signer(&[]),
                                space as u64,
                            )?;
                            let cpi_accounts = anchor_lang::system_program::Assign {
                                account_to_assign: game.to_account_info(),
                            };
                            let cpi_context = anchor_lang::context::CpiContext::new(
                                system_program.to_account_info(),
                                cpi_accounts,
                            );
                            anchor_lang::system_program::assign(
                                cpi_context.with_signer(&[]),
                                program_id,
                            )?;
                        }
                        anchor_lang::accounts::account::Account::try_from_unchecked(&game)?
                    } else {
                        anchor_lang::accounts::account::Account::try_from(&game)?
                    };
                    if false {
                        if space != actual_field.data_len() {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintSpace,
                            )
                            .with_account_name("game")
                            .with_values((space, actual_field.data_len())));
                        }
                        if actual_owner != program_id {
                            return Err(anchor_lang::error::Error::from(
                                anchor_lang::error::ErrorCode::ConstraintOwner,
                            )
                            .with_account_name("game")
                            .with_pubkeys((*actual_owner, *program_id)));
                        }
                        {
                            let required_lamports = __anchor_rent.minimum_balance(space);
                            if pa.to_account_info().lamports() < required_lamports {
                                return Err(anchor_lang::error::Error::from(
                                    anchor_lang::error::ErrorCode::ConstraintRentExempt,
                                )
                                .with_account_name("game"));
                            }
                        }
                    }
                    pa
                };
                if !game.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("game"));
                }
                if !game.to_account_info().is_signer {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintSigner,
                    )
                    .with_account_name("game"));
                }
                if !__anchor_rent.is_exempt(
                    game.to_account_info().lamports(),
                    game.to_account_info().try_data_len()?,
                ) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintRentExempt,
                    )
                    .with_account_name("game"));
                }
                if !player_one.to_account_info().is_writable {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::ErrorCode::ConstraintMut,
                    )
                    .with_account_name("player_one"));
                }
                Ok(SetupGame {
                    game,
                    player_one,
                    system_program,
                })
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountInfos<'info> for SetupGame<'info>
        where
            'info: 'info,
        {
            fn to_account_infos(
                &self,
            ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>> {
                let mut account_infos = ::alloc::vec::Vec::new();
                account_infos.extend(self.game.to_account_infos());
                account_infos.extend(self.player_one.to_account_infos());
                account_infos.extend(self.system_program.to_account_infos());
                account_infos
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::ToAccountMetas for SetupGame<'info> {
            fn to_account_metas(
                &self,
                is_signer: Option<bool>,
            ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                let mut account_metas = ::alloc::vec::Vec::new();
                account_metas.extend(self.game.to_account_metas(Some(true)));
                account_metas.extend(self.player_one.to_account_metas(None));
                account_metas.extend(self.system_program.to_account_metas(None));
                account_metas
            }
        }
        #[automatically_derived]
        impl<'info> anchor_lang::AccountsExit<'info> for SetupGame<'info>
        where
            'info: 'info,
        {
            fn exit(
                &self,
                program_id: &anchor_lang::solana_program::pubkey::Pubkey,
            ) -> anchor_lang::Result<()> {
                anchor_lang::AccountsExit::exit(&self.game, program_id)
                    .map_err(|e| e.with_account_name("game"))?;
                anchor_lang::AccountsExit::exit(&self.player_one, program_id)
                    .map_err(|e| e.with_account_name("player_one"))?;
                Ok(())
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is a Pubkey,
        /// instead of an `AccountInfo`. This is useful for clients that want
        /// to generate a list of accounts, without explicitly knowing the
        /// order all the fields should be in.
        ///
        /// To access the struct in this module, one should use the sibling
        /// `accounts` module (also generated), which re-exports this.
        pub(crate) mod __client_accounts_setup_game {
            use super::*;
            use anchor_lang::prelude::borsh;
            /// Generated client accounts for [`SetupGame`].
            pub struct SetupGame {
                pub game: anchor_lang::solana_program::pubkey::Pubkey,
                pub player_one: anchor_lang::solana_program::pubkey::Pubkey,
                pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
            }
            impl borsh::ser::BorshSerialize for SetupGame
            where
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
                anchor_lang::solana_program::pubkey::Pubkey: borsh::ser::BorshSerialize,
            {
                fn serialize<W: borsh::maybestd::io::Write>(
                    &self,
                    writer: &mut W,
                ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                    borsh::BorshSerialize::serialize(&self.game, writer)?;
                    borsh::BorshSerialize::serialize(&self.player_one, writer)?;
                    borsh::BorshSerialize::serialize(&self.system_program, writer)?;
                    Ok(())
                }
            }
            #[automatically_derived]
            impl anchor_lang::ToAccountMetas for SetupGame {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.game, true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        self.player_one,
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            self.system_program,
                            false,
                        ),
                    );
                    account_metas
                }
            }
        }
        /// An internal, Anchor generated module. This is used (as an
        /// implementation detail), to generate a CPI struct for a given
        /// `#[derive(Accounts)]` implementation, where each field is an
        /// AccountInfo.
        ///
        /// To access the struct in this module, one should use the sibling
        /// [`cpi::accounts`] module (also generated), which re-exports this.
        pub(crate) mod __cpi_client_accounts_setup_game {
            use super::*;
            /// Generated CPI struct of the accounts for [`SetupGame`].
            pub struct SetupGame<'info> {
                pub game: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub player_one: anchor_lang::solana_program::account_info::AccountInfo<'info>,
                pub system_program: anchor_lang::solana_program::account_info::AccountInfo<'info>,
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountMetas for SetupGame<'info> {
                fn to_account_metas(
                    &self,
                    is_signer: Option<bool>,
                ) -> Vec<anchor_lang::solana_program::instruction::AccountMeta> {
                    let mut account_metas = ::alloc::vec::Vec::new();
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.game),
                        true,
                    ));
                    account_metas.push(anchor_lang::solana_program::instruction::AccountMeta::new(
                        anchor_lang::Key::key(&self.player_one),
                        true,
                    ));
                    account_metas.push(
                        anchor_lang::solana_program::instruction::AccountMeta::new_readonly(
                            anchor_lang::Key::key(&self.system_program),
                            false,
                        ),
                    );
                    account_metas
                }
            }
            #[automatically_derived]
            impl<'info> anchor_lang::ToAccountInfos<'info> for SetupGame<'info> {
                fn to_account_infos(
                    &self,
                ) -> Vec<anchor_lang::solana_program::account_info::AccountInfo<'info>>
                {
                    let mut account_infos = ::alloc::vec::Vec::new();
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(&self.game));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.player_one,
                    ));
                    account_infos.push(anchor_lang::ToAccountInfo::to_account_info(
                        &self.system_program,
                    ));
                    account_infos
                }
            }
        }
    }
}
pub mod state {
    pub use game::*;
    pub mod game {
        use crate::errors::TicTacToeError;
        use anchor_lang::prelude::*;
        use num_derive::*;
        use num_traits::*;
        pub struct Game {
            players: [Pubkey; 2],
            turn: u8,
            board: [[Option<Sign>; 3]; 3],
            state: GameState,
        }
        impl borsh::ser::BorshSerialize for Game
        where
            [Pubkey; 2]: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
            [[Option<Sign>; 3]; 3]: borsh::ser::BorshSerialize,
            GameState: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.players, writer)?;
                borsh::BorshSerialize::serialize(&self.turn, writer)?;
                borsh::BorshSerialize::serialize(&self.board, writer)?;
                borsh::BorshSerialize::serialize(&self.state, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Game
        where
            [Pubkey; 2]: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
            [[Option<Sign>; 3]; 3]: borsh::BorshDeserialize,
            GameState: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    players: borsh::BorshDeserialize::deserialize(buf)?,
                    turn: borsh::BorshDeserialize::deserialize(buf)?,
                    board: borsh::BorshDeserialize::deserialize(buf)?,
                    state: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Game {
            #[inline]
            fn clone(&self) -> Game {
                match *self {
                    Game {
                        players: ref __self_0_0,
                        turn: ref __self_0_1,
                        board: ref __self_0_2,
                        state: ref __self_0_3,
                    } => Game {
                        players: ::core::clone::Clone::clone(&(*__self_0_0)),
                        turn: ::core::clone::Clone::clone(&(*__self_0_1)),
                        board: ::core::clone::Clone::clone(&(*__self_0_2)),
                        state: ::core::clone::Clone::clone(&(*__self_0_3)),
                    },
                }
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountSerialize for Game {
            fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
                if writer
                    .write_all(&[27, 90, 166, 125, 74, 100, 121, 18])
                    .is_err()
                {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
                if AnchorSerialize::serialize(self, writer).is_err() {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
                Ok(())
            }
        }
        #[automatically_derived]
        impl anchor_lang::AccountDeserialize for Game {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                if buf.len() < [27, 90, 166, 125, 74, 100, 121, 18].len() {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorNotFound.into());
                }
                let given_disc = &buf[..8];
                if &[27, 90, 166, 125, 74, 100, 121, 18] != given_disc {
                    return Err(anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch.into());
                }
                Self::try_deserialize_unchecked(buf)
            }
            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data: &[u8] = &buf[8..];
                AnchorDeserialize::deserialize(&mut data)
                    .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
            }
        }
        #[automatically_derived]
        impl anchor_lang::Discriminator for Game {
            fn discriminator() -> [u8; 8] {
                [27, 90, 166, 125, 74, 100, 121, 18]
            }
        }
        #[automatically_derived]
        impl anchor_lang::Owner for Game {
            fn owner() -> Pubkey {
                crate::ID
            }
        }
        impl Game {
            pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1)) + (32 + 1);
            pub fn start(&mut self, players: [Pubkey; 2]) -> Result<()> {
                if self.turn != 0 {
                    return Err(
                        anchor_lang::error::Error::from(anchor_lang::error::AnchorError {
                            error_name: TicTacToeError::GameAlreadyStarted.name(),
                            error_code_number: TicTacToeError::GameAlreadyStarted.into(),
                            error_msg: TicTacToeError::GameAlreadyStarted.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/tic-tac-toe/src/state/game.rs",
                                    line: 18u32,
                                },
                            )),
                            compared_values: None,
                        })
                        .with_values((self.turn, 0)),
                    );
                };
                self.players = players;
                self.turn = 1;
                Ok(())
            }
            pub fn is_active(&self) -> bool {
                self.state == GameState::Active
            }
            fn current_player_index(&self) -> usize {
                ((self.turn - 1) % 2) as usize
            }
            pub fn current_player(&self) -> Pubkey {
                self.players[self.current_player_index()]
            }
            pub fn play(&mut self, tile: &Tile) -> Result<()> {
                if !(self.is_active()) {
                    return Err(anchor_lang::error::Error::from(
                        anchor_lang::error::AnchorError {
                            error_name: TicTacToeError::GameAlreadyOver.name(),
                            error_code_number: TicTacToeError::GameAlreadyOver.into(),
                            error_msg: TicTacToeError::GameAlreadyOver.to_string(),
                            error_origin: Some(anchor_lang::error::ErrorOrigin::Source(
                                anchor_lang::error::Source {
                                    filename: "programs/tic-tac-toe/src/state/game.rs",
                                    line: 37u32,
                                },
                            )),
                            compared_values: None,
                        },
                    ));
                };
                match tile {
                    tile @ Tile {
                        row: 0..=2,
                        column: 0..=2,
                    } => match self.board[tile.row as usize][tile.column as usize] {
                        Some(_) => return Err(TicTacToeError::TileAlreadySet.into()),
                        None => {
                            self.board[tile.row as usize][tile.column as usize] =
                                Some(Sign::from_usize(self.current_player_index()).unwrap());
                        }
                    },
                    _ => return Err(TicTacToeError::TileOutOfBounds.into()),
                }
                self.update_state();
                if GameState::Active == self.state {
                    self.turn += 1;
                }
                Ok(())
            }
            fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
                let [first, second, third] = trio;
                self.board[first.0][first.1].is_some()
                    && self.board[first.0][first.1] == self.board[second.0][second.1]
                    && self.board[first.0][first.1] == self.board[third.0][third.1]
            }
            fn update_state(&mut self) {
                for i in 0..=2 {
                    if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
                        self.state = GameState::Won {
                            winner: self.current_player(),
                        };
                        return;
                    }
                    if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
                        self.state = GameState::Won {
                            winner: self.current_player(),
                        };
                        return;
                    }
                }
                if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
                    || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
                {
                    self.state = GameState::Won {
                        winner: self.current_player(),
                    };
                    return;
                }
                for row in 0..=2 {
                    for column in 0..=2 {
                        if self.board[row][column].is_none() {
                            return;
                        }
                    }
                }
                self.state = GameState::Tie;
            }
        }
        pub enum GameState {
            Active,
            Tie,
            Won { winner: Pubkey },
        }
        impl borsh::ser::BorshSerialize for GameState
        where
            Pubkey: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    GameState::Active => 0u8,
                    GameState::Tie => 1u8,
                    GameState::Won { .. } => 2u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    GameState::Active => {}
                    GameState::Tie => {}
                    GameState::Won { winner } => {
                        borsh::BorshSerialize::serialize(winner, writer)?;
                    }
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for GameState
        where
            Pubkey: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> core::result::Result<Self, borsh::maybestd::io::Error> {
                let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
                let return_value = match variant_idx {
                    0u8 => GameState::Active,
                    1u8 => GameState::Tie,
                    2u8 => GameState::Won {
                        winner: borsh::BorshDeserialize::deserialize(buf)?,
                    },
                    _ => {
                        let msg = {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Unexpected variant index: "],
                                &[::core::fmt::ArgumentV1::new_debug(&variant_idx)],
                            ));
                            res
                        };
                        return Err(borsh::maybestd::io::Error::new(
                            borsh::maybestd::io::ErrorKind::InvalidInput,
                            msg,
                        ));
                    }
                };
                Ok(return_value)
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for GameState {
            #[inline]
            fn clone(&self) -> GameState {
                match (&*self,) {
                    (&GameState::Active,) => GameState::Active,
                    (&GameState::Tie,) => GameState::Tie,
                    (&GameState::Won {
                        winner: ref __self_0,
                    },) => GameState::Won {
                        winner: ::core::clone::Clone::clone(&(*__self_0)),
                    },
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for GameState {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for GameState {
            #[inline]
            fn eq(&self, other: &GameState) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (
                                &GameState::Won {
                                    winner: ref __self_0,
                                },
                                &GameState::Won {
                                    winner: ref __arg_1_0,
                                },
                            ) => (*__self_0) == (*__arg_1_0),
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
            #[inline]
            fn ne(&self, other: &GameState) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            (
                                &GameState::Won {
                                    winner: ref __self_0,
                                },
                                &GameState::Won {
                                    winner: ref __arg_1_0,
                                },
                            ) => (*__self_0) != (*__arg_1_0),
                            _ => false,
                        }
                    } else {
                        true
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for GameState {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for GameState {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {
                    let _: ::core::cmp::AssertParamIsEq<Pubkey>;
                }
            }
        }
        pub enum Sign {
            X,
            O,
        }
        impl borsh::ser::BorshSerialize for Sign {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> core::result::Result<(), borsh::maybestd::io::Error> {
                let variant_idx: u8 = match self {
                    Sign::X => 0u8,
                    Sign::O => 1u8,
                };
                writer.write_all(&variant_idx.to_le_bytes())?;
                match self {
                    Sign::X => {}
                    Sign::O => {}
                }
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Sign {
            fn deserialize(
                buf: &mut &[u8],
            ) -> core::result::Result<Self, borsh::maybestd::io::Error> {
                let variant_idx: u8 = borsh::BorshDeserialize::deserialize(buf)?;
                let return_value = match variant_idx {
                    0u8 => Sign::X,
                    1u8 => Sign::O,
                    _ => {
                        let msg = {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Unexpected variant index: "],
                                &[::core::fmt::ArgumentV1::new_debug(&variant_idx)],
                            ));
                            res
                        };
                        return Err(borsh::maybestd::io::Error::new(
                            borsh::maybestd::io::ErrorKind::InvalidInput,
                            msg,
                        ));
                    }
                };
                Ok(return_value)
            }
        }
        #[allow(non_upper_case_globals, unused_qualifications)]
        const _IMPL_NUM_FromPrimitive_FOR_Sign: () = {
            #[allow(clippy::useless_attribute)]
            #[allow(rust_2018_idioms)]
            extern crate num_traits as _num_traits;
            impl _num_traits::FromPrimitive for Sign {
                #[allow(trivial_numeric_casts)]
                #[inline]
                fn from_i64(n: i64) -> Option<Self> {
                    if n == Sign::X as i64 {
                        Some(Sign::X)
                    } else if n == Sign::O as i64 {
                        Some(Sign::O)
                    } else {
                        None
                    }
                }
                #[inline]
                fn from_u64(n: u64) -> Option<Self> {
                    Self::from_i64(n as i64)
                }
            }
        };
        #[allow(non_upper_case_globals, unused_qualifications)]
        const _IMPL_NUM_ToPrimitive_FOR_Sign: () = {
            #[allow(clippy::useless_attribute)]
            #[allow(rust_2018_idioms)]
            extern crate num_traits as _num_traits;
            impl _num_traits::ToPrimitive for Sign {
                #[inline]
                #[allow(trivial_numeric_casts)]
                fn to_i64(&self) -> Option<i64> {
                    Some(match *self {
                        Sign::X => Sign::X as i64,
                        Sign::O => Sign::O as i64,
                    })
                }
                #[inline]
                fn to_u64(&self) -> Option<u64> {
                    self.to_i64().map(|x| x as u64)
                }
            }
        };
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::marker::Copy for Sign {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::clone::Clone for Sign {
            #[inline]
            fn clone(&self) -> Sign {
                {
                    *self
                }
            }
        }
        impl ::core::marker::StructuralPartialEq for Sign {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for Sign {
            #[inline]
            fn eq(&self, other: &Sign) -> bool {
                {
                    let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                    let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) {
                            _ => true,
                        }
                    } else {
                        false
                    }
                }
            }
        }
        impl ::core::marker::StructuralEq for Sign {}
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::Eq for Sign {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                {}
            }
        }
        pub struct Tile {
            row: u8,
            column: u8,
        }
        impl borsh::ser::BorshSerialize for Tile
        where
            u8: borsh::ser::BorshSerialize,
            u8: borsh::ser::BorshSerialize,
        {
            fn serialize<W: borsh::maybestd::io::Write>(
                &self,
                writer: &mut W,
            ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
                borsh::BorshSerialize::serialize(&self.row, writer)?;
                borsh::BorshSerialize::serialize(&self.column, writer)?;
                Ok(())
            }
        }
        impl borsh::de::BorshDeserialize for Tile
        where
            u8: borsh::BorshDeserialize,
            u8: borsh::BorshDeserialize,
        {
            fn deserialize(
                buf: &mut &[u8],
            ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
                Ok(Self {
                    row: borsh::BorshDeserialize::deserialize(buf)?,
                    column: borsh::BorshDeserialize::deserialize(buf)?,
                })
            }
        }
    }
}
/// The static program ID
pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
        84u8, 90u8, 245u8, 136u8, 213u8, 59u8, 169u8, 228u8, 49u8, 85u8, 180u8, 199u8, 213u8, 87u8,
        158u8, 50u8, 178u8, 126u8, 1u8, 247u8, 33u8, 97u8, 209u8, 189u8, 197u8, 213u8, 51u8, 138u8,
        135u8, 128u8, 24u8, 23u8,
    ]);
/// Confirms that a given pubkey is equivalent to the program ID
pub fn check_id(id: &anchor_lang::solana_program::pubkey::Pubkey) -> bool {
    id == &ID
}
/// Returns the program ID
pub fn id() -> anchor_lang::solana_program::pubkey::Pubkey {
    ID
}
use self::tic_tac_toe::*;
/// # Safety
#[no_mangle]
pub unsafe extern "C" fn entrypoint(input: *mut u8) -> u64 {
    let (program_id, accounts, instruction_data) =
        unsafe { ::solana_program::entrypoint::deserialize(input) };
    match entry(&program_id, &accounts, &instruction_data) {
        Ok(()) => ::solana_program::entrypoint::SUCCESS,
        Err(error) => error.into(),
    }
}
/// The Anchor codegen exposes a programming model where a user defines
/// a set of methods inside of a `#[program]` module in a way similar
/// to writing RPC request handlers. The macro then generates a bunch of
/// code wrapping these user defined methods into something that can be
/// executed on Solana.
///
/// These methods fall into one of three categories, each of which
/// can be considered a different "namespace" of the program.
///
/// 1) Global methods - regular methods inside of the `#[program]`.
/// 2) State methods - associated methods inside a `#[state]` struct.
/// 3) Interface methods - methods inside a strait struct's
///    implementation of an `#[interface]` trait.
///
/// Care must be taken by the codegen to prevent collisions between
/// methods in these different namespaces. For this reason, Anchor uses
/// a variant of sighash to perform method dispatch, rather than
/// something like a simple enum variant discriminator.
///
/// The execution flow of the generated code can be roughly outlined:
///
/// * Start program via the entrypoint.
/// * Strip method identifier off the first 8 bytes of the instruction
///   data and invoke the identified method. The method identifier
///   is a variant of sighash. See docs.rs for `anchor_lang` for details.
/// * If the method identifier is an IDL identifier, execute the IDL
///   instructions, which are a special set of hardcoded instructions
///   baked into every Anchor program. Then exit.
/// * Otherwise, the method identifier is for a user defined
///   instruction, i.e., one of the methods in the user defined
///   `#[program]` module. Perform method dispatch, i.e., execute the
///   big match statement mapping method identifier to method handler
///   wrapper.
/// * Run the method handler wrapper. This wraps the code the user
///   actually wrote, deserializing the accounts, constructing the
///   context, invoking the user's code, and finally running the exit
///   routine, which typically persists account changes.
///
/// The `entry` function here, defines the standard entry to a Solana
/// program, where execution begins.
pub fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::solana_program::entrypoint::ProgramResult {
    try_entry(program_id, accounts, data).map_err(|e| {
        e.log();
        e.into()
    })
}
fn try_entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> anchor_lang::Result<()> {
    if *program_id != ID {
        return Err(anchor_lang::error::ErrorCode::DeclaredProgramIdMismatch.into());
    }
    if data.len() < 8 {
        return Err(anchor_lang::error::ErrorCode::InstructionMissing.into());
    }
    dispatch(program_id, accounts, data)
}
/// Module representing the program.
pub mod program {
    use super::*;
    /// Type representing the program.
    pub struct TicTacToe;
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TicTacToe {
        #[inline]
        fn clone(&self) -> TicTacToe {
            match *self {
                TicTacToe => TicTacToe,
            }
        }
    }
    impl anchor_lang::Id for TicTacToe {
        fn id() -> Pubkey {
            ID
        }
    }
}
/// Performs method dispatch.
///
/// Each method in an anchor program is uniquely defined by a namespace
/// and a rust identifier (i.e., the name given to the method). These
/// two pieces can be combined to creater a method identifier,
/// specifically, Anchor uses
///
/// Sha256("<namespace>::<rust-identifier>")[..8],
///
/// where the namespace can be one of three types. 1) "global" for a
/// regular instruction, 2) "state" for a state struct instruction
/// handler and 3) a trait namespace (used in combination with the
/// `#[interface]` attribute), which is defined by the trait name, e..
/// `MyTrait`.
///
/// With this 8 byte identifier, Anchor performs method dispatch,
/// matching the given 8 byte identifier to the associated method
/// handler, which leads to user defined code being eventually invoked.
fn dispatch(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> anchor_lang::Result<()> {
    let mut ix_data: &[u8] = data;
    let sighash: [u8; 8] = {
        let mut sighash: [u8; 8] = [0; 8];
        sighash.copy_from_slice(&ix_data[..8]);
        ix_data = &ix_data[8..];
        sighash
    };
    if true {
        if sighash == anchor_lang::idl::IDL_IX_TAG.to_le_bytes() {
            return __private::__idl::__idl_dispatch(program_id, accounts, &ix_data);
        }
    }
    match sighash {
        [180, 218, 128, 75, 58, 222, 35, 82] => {
            __private::__global::setup_game(program_id, accounts, ix_data)
        }
        [213, 157, 193, 142, 228, 56, 248, 150] => {
            __private::__global::play(program_id, accounts, ix_data)
        }
        _ => Err(anchor_lang::error::ErrorCode::InstructionFallbackNotFound.into()),
    }
}
/// Create a private module to not clutter the program's namespace.
/// Defines an entrypoint for each individual instruction handler
/// wrapper.
mod __private {
    use super::*;
    /// __idl mod defines handlers for injected Anchor IDL instructions.
    pub mod __idl {
        use super::*;
        #[inline(never)]
        #[cfg(not(feature = "no-idl"))]
        pub fn __idl_dispatch(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            idl_ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            let mut accounts = accounts;
            let mut data: &[u8] = idl_ix_data;
            let ix = anchor_lang::idl::IdlInstruction::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            match ix {
                anchor_lang::idl::IdlInstruction::Create { data_len } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_account(program_id, &mut accounts, data_len)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::CreateBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlCreateBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_create_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::Write { data } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_write(program_id, &mut accounts, data)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetAuthority { new_authority } => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlAccounts::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_authority(program_id, &mut accounts, new_authority)?;
                    accounts.exit(program_id)?;
                }
                anchor_lang::idl::IdlInstruction::SetBuffer => {
                    let mut bumps = std::collections::BTreeMap::new();
                    let mut accounts = anchor_lang::idl::IdlSetBuffer::try_accounts(
                        program_id,
                        &mut accounts,
                        &[],
                        &mut bumps,
                    )?;
                    __idl_set_buffer(program_id, &mut accounts)?;
                    accounts.exit(program_id)?;
                }
            }
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_account(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateAccounts,
            data_len: u64,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateAccount");
            if program_id != accounts.program.key {
                return Err(anchor_lang::error::ErrorCode::IdlInstructionInvalidProgram.into());
            }
            let from = accounts.from.key;
            let (base, nonce) = Pubkey::find_program_address(&[], program_id);
            let seed = anchor_lang::idl::IdlAccount::seed();
            let owner = accounts.program.key;
            let to = Pubkey::create_with_seed(&base, seed, owner).unwrap();
            let space = 8 + 32 + 4 + data_len as usize;
            let rent = Rent::get()?;
            let lamports = rent.minimum_balance(space);
            let seeds = &[&[nonce][..]];
            let ix = anchor_lang::solana_program::system_instruction::create_account_with_seed(
                from,
                &to,
                &base,
                seed,
                lamports,
                space as u64,
                owner,
            );
            anchor_lang::solana_program::program::invoke_signed(
                &ix,
                &[
                    accounts.from.clone(),
                    accounts.to.clone(),
                    accounts.base.clone(),
                    accounts.system_program.clone(),
                ],
                &[seeds],
            )?;
            let mut idl_account = {
                let mut account_data = accounts.to.try_borrow_data()?;
                let mut account_data_slice: &[u8] = &account_data;
                anchor_lang::idl::IdlAccount::try_deserialize_unchecked(&mut account_data_slice)?
            };
            idl_account.authority = *accounts.from.key;
            let mut data = accounts.to.try_borrow_mut_data()?;
            let dst: &mut [u8] = &mut data;
            let mut cursor = std::io::Cursor::new(dst);
            idl_account.try_serialize(&mut cursor)?;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_create_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlCreateBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlCreateBuffer");
            let mut buffer = &mut accounts.buffer;
            buffer.authority = *accounts.authority.key;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_write(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            idl_data: Vec<u8>,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlWrite");
            let mut idl = &mut accounts.idl;
            idl.data.extend(idl_data);
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_authority(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlAccounts,
            new_authority: Pubkey,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetAuthority");
            accounts.idl.authority = new_authority;
            Ok(())
        }
        #[inline(never)]
        pub fn __idl_set_buffer(
            program_id: &Pubkey,
            accounts: &mut anchor_lang::idl::IdlSetBuffer,
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: IdlSetBuffer");
            accounts.idl.data = accounts.buffer.data.clone();
            Ok(())
        }
    }
    /// __state mod defines wrapped handlers for state instructions.
    pub mod __state {
        use super::*;
    }
    /// __interface mod defines wrapped handlers for `#[interface]` trait
    /// implementations.
    pub mod __interface {
        use super::*;
    }
    /// __global mod defines wrapped handlers for global instructions.
    pub mod __global {
        use super::*;
        #[inline(never)]
        pub fn setup_game(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: SetupGame");
            let ix = instruction::SetupGame::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::SetupGame { player_two } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts = SetupGame::try_accounts(
                program_id,
                &mut remaining_accounts,
                ix_data,
                &mut __bumps,
            )?;
            let result = tic_tac_toe::setup_game(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                player_two,
            )?;
            accounts.exit(program_id)
        }
        #[inline(never)]
        pub fn play(
            program_id: &Pubkey,
            accounts: &[AccountInfo],
            ix_data: &[u8],
        ) -> anchor_lang::Result<()> {
            ::solana_program::log::sol_log("Instruction: Play");
            let ix = instruction::Play::deserialize(&mut &ix_data[..])
                .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
            let instruction::Play { tile } = ix;
            let mut __bumps = std::collections::BTreeMap::new();
            let mut remaining_accounts: &[AccountInfo] = accounts;
            let mut accounts =
                Play::try_accounts(program_id, &mut remaining_accounts, ix_data, &mut __bumps)?;
            let result = tic_tac_toe::play(
                anchor_lang::context::Context::new(
                    program_id,
                    &mut accounts,
                    remaining_accounts,
                    __bumps,
                ),
                tile,
            )?;
            accounts.exit(program_id)
        }
    }
}
pub mod tic_tac_toe {
    use super::*;
    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        instructions::setup_game::setup_game(ctx, player_two)
    }
    pub fn play(ctx: Context<Play>, tile: Tile) -> Result<()> {
        instructions::play::play(ctx, tile)
    }
}
/// An Anchor generated module containing the program's set of
/// instructions, where each method handler in the `#[program]` mod is
/// associated with a struct defining the input arguments to the
/// method. These should be used directly, when one wants to serialize
/// Anchor instruction data, for example, when speciying
/// instructions on a client.
pub mod instruction {
    use super::*;
    /// Instruction struct definitions for `#[state]` methods.
    pub mod state {
        use super::*;
    }
    /// Instruction.
    pub struct SetupGame {
        pub player_two: Pubkey,
    }
    impl borsh::ser::BorshSerialize for SetupGame
    where
        Pubkey: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.player_two, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for SetupGame
    where
        Pubkey: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                player_two: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for SetupGame {
        fn data(&self) -> Vec<u8> {
            let mut d = [180, 218, 128, 75, 58, 222, 35, 82].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
    /// Instruction.
    pub struct Play {
        pub tile: Tile,
    }
    impl borsh::ser::BorshSerialize for Play
    where
        Tile: borsh::ser::BorshSerialize,
    {
        fn serialize<W: borsh::maybestd::io::Write>(
            &self,
            writer: &mut W,
        ) -> ::core::result::Result<(), borsh::maybestd::io::Error> {
            borsh::BorshSerialize::serialize(&self.tile, writer)?;
            Ok(())
        }
    }
    impl borsh::de::BorshDeserialize for Play
    where
        Tile: borsh::BorshDeserialize,
    {
        fn deserialize(
            buf: &mut &[u8],
        ) -> ::core::result::Result<Self, borsh::maybestd::io::Error> {
            Ok(Self {
                tile: borsh::BorshDeserialize::deserialize(buf)?,
            })
        }
    }
    impl anchor_lang::InstructionData for Play {
        fn data(&self) -> Vec<u8> {
            let mut d = [213, 157, 193, 142, 228, 56, 248, 150].to_vec();
            d.append(&mut self.try_to_vec().expect("Should always serialize"));
            d
        }
    }
}
/// An Anchor generated module, providing a set of structs
/// mirroring the structs deriving `Accounts`, where each field is
/// a `Pubkey`. This is useful for specifying accounts for a client.
pub mod accounts {
    pub use crate::__client_accounts_play::*;
    pub use crate::__client_accounts_setup_game::*;
}
