use anchor_lang::prelude::*;
use anchor_spl::token;
use crate::constants::{MARKET_SETTING,SETTING_SIZE};
use crate::state::order::Settings;

pub fn process_init_settings(
    ctx: Context<InitSettings>,
    support_coins: Vec<Pubkey>,
    fee_ratio: u16,
) -> Result<()> {
    msg!("In init state");
    ctx.accounts.setting_account.support_coins = support_coins;
    ctx.accounts.setting_account.authority = ctx.accounts.authority.key();
    ctx.accounts.setting_account.fee_ratio = fee_ratio;
    Ok(())
}

#[derive(Accounts)]
pub struct InitSettings<'info> {
    #[account(
    init,
    seeds = [MARKET_SETTING.as_ref()],
    bump,
    payer = authority,
    space = SETTING_SIZE,
    )]
    pub setting_account: Account<'info, Settings>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,

}