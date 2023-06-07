use crate::constants::PROVIDER_CONFIG_SEED;
use crate::state::ProviderConfigAccount;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct UpdateProvider<'info> {
    /// CHECK: this account to create provider config account and fill data to it
    pub provider: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [PROVIDER_CONFIG_SEED, admin.key().as_ref(), provider.key().as_ref()],
        bump = provider_config_account.bump,
        constraint = provider_config_account.deactivate == false
    )]
    pub provider_config_account: Account<'info, ProviderConfigAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_update_provider<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateProvider<'info>>
) -> Result<()> {
    // TODO: Logic update provider
    Ok(())
}
