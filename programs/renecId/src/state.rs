use anchor_lang::prelude::*;

// Provider config account
// this account hold all config about provider
// that support us easy to manage and verify provider
#[account]
pub struct ProviderConfigAccount {
    pub provider: Pubkey,
    pub deactivate: bool,
    pub admin: Pubkey,
    pub bump: u8,
}

impl ProviderConfigAccount {
    pub const LEN: usize = 8;
}

// user kyc account
// this account hold KYC info of an user
#[account]
pub struct UserKycAccount {
    pub user: Pubkey,
    pub name: String,
    pub document_id: String,
    pub country: String,
    pub date_of_birth: String,
    pub date_of_expired: String,
    pub gender: String,
    pub kyc_level: u8,
    pub is_expired: bool,
    // deactivate or activate user
    pub deactivate: bool,
    // admin who has permission to deactivate or activate user
    pub admin: Pubkey,
    // Provider config address
    pub provider: Pubkey,
    pub bump: u8,
}

impl UserKycAccount {
    pub const LEN: usize = 8;
}
