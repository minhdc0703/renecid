use crate::constants::{ USER_KYC_ACCOUNT_SEED };
use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct UpdateUser<'info> {
    #[account(mut,
        seeds = [USER_KYC_ACCOUNT_SEED, admin.key().as_ref(), user.key().as_ref()],
        bump=user_kyc_account.bump,
        constraint = user_kyc_account.deactivate == false
    )]
    pub user_kyc_account: Account<'info, UserKycAccount>,
    // admin who has permission create new user. Without admin sign transaction user will never create
    pub admin: Signer<'info>,
    pub provider: Signer<'info>,
    /// CHECK: this account to verify user kyc account
    pub user: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler_update_user<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateUser<'info>>
) -> Result<()> {
    // TODO: Logic create user
    Ok(())
}
