use anchor_lang::prelude::*;

use crate::constants::MARKET_SETTING;
use crate::state::order::Settings;

#[derive(Accounts)]
pub struct UpdateSettings<'info> {
    #[account(mut, has_one = authority, seeds = [MARKET_SETTING.as_ref()],bump)]
    pub setting_account: Account<'info, Settings>,
    pub authority: Signer<'info>,
}

pub fn process_update_settings(
    ctx: Context<UpdateSettings>,
    support_coins: Vec<Pubkey>,
    fee_ratio: u16,
    new_authority: Option<Pubkey>,
) -> Result<()> {
    msg!("update state");
    ctx.accounts.setting_account.support_coins = support_coins;
    if let Some(authority) = new_authority {
        ctx.accounts.setting_account.authority = authority;
    }
    ctx.accounts.setting_account.fee_ratio = fee_ratio;
    Ok(())
}
