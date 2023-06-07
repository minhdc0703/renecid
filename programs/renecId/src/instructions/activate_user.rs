use crate::constants::{ USER_KYC_ACCOUNT_SEED };
use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct ActivateUser<'info> {
    #[account(
        mut,
        seeds = [USER_KYC_ACCOUNT_SEED, admin.key().as_ref(), user.key().as_ref()],
        bump=user_kyc_account.bump
    )]
    pub user_kyc_account: Account<'info, UserKycAccount>,
    /// CHECK: this account use to check user kyc account
    pub user: AccountInfo<'info>,
    // admin who has permission create new user. Without admin sign transaction user will never create
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_activate_user<'info>(
    ctx: Context<'_, '_, '_, 'info, ActivateUser<'info>>
) -> Result<()> {
    // TODO: Logic create user
    Ok(())
}
