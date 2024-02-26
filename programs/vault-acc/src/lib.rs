use anchor_lang::prelude::*;

declare_id!("9pU1ESxdgeWPKjQwg6R7vuVom8rno7L8CNb76EUh6vVN");

#[program]
pub mod vault_acc {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.auth_bump = ctx.bumps.auth;
        ctx.accounts.state.vault_bump = ctx.bumps.vault;
        ctx.accounts.state.state_bump = ctx.bumps.state;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        init,
        payer=owner,
        space=VaultState::LEN,
        seeds=[b"state", owner.key().as_ref()],
        bump
    )]
    state: Account<'info, VaultState>,

    #[account(
        seeds=[b"auth", state.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,

    #[account(
        seeds=[b"vault", state.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,

    system_program: Program<'info, System>
}

#[account]
pub struct VaultState {
    auth_bump: u8,
    vault_bump: u8,
    state_bump: u8,
}

impl VaultState {
    const LEN: usize = 8 + 3 * 1;
}