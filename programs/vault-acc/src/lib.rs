use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer as SPLTransfer, transfer as spl_transfer};

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

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info()
        };

        let cpi = CpiContext::new(
            ctx.accounts.system_program.to_account_info(), 
            accounts
        );

        transfer(cpi, amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        let seeds = &[
            b"auth",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.auth_bump]
        ];

        let pda_signer = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(), 
            accounts, 
            pda_signer
        );

        transfer(cpi, amount)
    }

    pub fn deposit_spl(ctx: Context<SPLDesposit>, amount: u64) -> Result<()> {
        let accounts = SPLTransfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            accounts
        );

        spl_transfer(cpi, amount)
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

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    #[account(
        mut,
        seeds=[b"state", owner.key().as_ref()],
        bump=state.state_bump,
    )]
    state: Account<'info, VaultState>,

    #[account(
        seeds=[b"auth", state.key().as_ref()],
        bump=state.auth_bump,
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,

    #[account(
        seeds=[b"vault", state.key().as_ref()],
        bump=state.vault_bump
    )]
    vault: SystemAccount<'info>,

    system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct SPLDesposit<'info> {
    #[account(mut)]
    owner: Signer<'info>,

    mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=owner,
    )]
    owner_ata: Account<'info, TokenAccount>,

    #[account(
        seeds=[b"auth", state.key().as_ref()],
        bump=state.auth_bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,

    #[account(
        seeds=[b"state", owner.key().as_ref()],
        bump=state.state_bump
    )]
    state: Account<'info, VaultState>,

    #[account(
        init,
        payer=owner,
        seeds=[b"spl_vault", state.key().as_ref()],
        bump,
        token::mint=mint,
        token::authority=auth,
    )]
    vault: Account<'info, TokenAccount>,

    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
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