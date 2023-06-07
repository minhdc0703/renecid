use crate::constants::{PROVIDER_CONFIG_SEED, USER_KYC_ACCOUNT_SEED};
use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct CreateUser<'info> {
    #[account(
        seeds = [PROVIDER_CONFIG_SEED, admin.key().as_ref(), provider.key().as_ref()],
        bump = provider_config_account.bump,
        constraint = provider_config_account.deactivate == false
    )]
    pub provider_config_account: Account<'info, ProviderConfigAccount>,
    #[account(
        init,
        payer = user,
        seeds = [USER_KYC_ACCOUNT_SEED, admin.key().as_ref(), user.key().as_ref()],
        bump,
        space = ProviderConfigAccount::LEN
    )]
    pub user_kyc_account: Account<'info, UserKycAccount>,
    // admin who has permission create new user. Without admin sign transaction user will never create
    pub admin: Signer<'info>,
    // provider who has responsibilities to make sure that user kyc data is valid
    // Can not fake a provider to provide invalid kyc data
    pub provider: Signer<'info>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_create_user<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateUser<'info>>
) -> Result<()> {
    // TODO: Logic create user
    Ok(())
}
