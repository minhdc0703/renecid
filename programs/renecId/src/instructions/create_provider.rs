use crate::constants::PROVIDER_CONFIG_SEED;
use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct CreateProvider<'info> {
    /// CHECK: this account to create provider config account and fill data to it
    pub provider: AccountInfo<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [PROVIDER_CONFIG_SEED, admin.key().as_ref(), provider.key().as_ref()],
        bump,
        space = ProviderConfigAccount::LEN
    )]
    pub provider_config_account: Account<'info, ProviderConfigAccount>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_create_provider<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateProvider<'info>>
) -> Result<()> {
    // TODO: Logic create provider
    Ok(())
}
