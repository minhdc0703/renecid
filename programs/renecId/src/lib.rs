use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod constants;

use crate::instructions::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod renec_id {
    use super::*;

    pub fn create_provider<'info>(
        ctx: Context<'_, '_, '_, 'info, CreateProvider<'info>>
    ) -> Result<()> {
        handler_create_provider(ctx)?;
        Ok(())
    }

    pub fn create_user<'info>(ctx: Context<'_, '_, '_, 'info, CreateUser<'info>>) -> Result<()> {
        handler_create_user(ctx)?;
        Ok(())
    }

    pub fn update_user<'info>(ctx: Context<'_, '_, '_, 'info, UpdateUser<'info>>) -> Result<()> {
        handler_update_user(ctx)?;
        Ok(())
    }

    pub fn update_provider<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateProvider<'info>>
    ) -> Result<()> {
        handler_update_provider(ctx)?;
        Ok(())
    }

    pub fn activate_provider<'info>(
        ctx: Context<'_, '_, '_, 'info, ActivateProvider<'info>>
    ) -> Result<()> {
        handler_activate_provider(ctx)?;
        Ok(())
    }

    pub fn activate_user<'info>(
        ctx: Context<'_, '_, '_, 'info, ActivateUser<'info>>
    ) -> Result<()> {
        handler_activate_user(ctx)?;
        Ok(())
    }

    pub fn deactivate_provider<'info>(
        ctx: Context<'_, '_, '_, 'info, DeactivateProvider<'info>>
    ) -> Result<()> {
        handler_deactivate_provider(ctx)?;
        Ok(())
    }

    pub fn deactivate_user<'info>(
        ctx: Context<'_, '_, '_, 'info, DeactivateUser<'info>>
    ) -> Result<()> {
        handler_deactivate_user(ctx)?;
        Ok(())
    }
}
