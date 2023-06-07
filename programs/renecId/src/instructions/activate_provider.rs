use crate::constants::PROVIDER_CONFIG_SEED;
use crate::state::ProviderConfigAccount;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct ActivateProvider<'info> {
    /// CHECK: this account use to check provider config
    pub provider: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [PROVIDER_CONFIG_SEED, admin.key().as_ref(), provider.key().as_ref()],
        bump = provider_config_account.bump
    )]
    pub provider_config_account: Account<'info, ProviderConfigAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_activate_provider<'info>(
    ctx: Context<'_, '_, '_, 'info, ActivateProvider<'info>>
) -> Result<()> {
    // TODO: Logic deactivate provider
    Ok(())
}
