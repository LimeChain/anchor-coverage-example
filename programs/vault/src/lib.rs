#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use anchor_lang::system_program::{create_account, transfer, CreateAccount, Transfer};
mod constants;

declare_id!("Ct4gVR9ggiLraQKcZeNs2fkbtDtCSxowC9KjCDcnSDyC");

#[program]
pub mod vault {

    use crate::constants::JANUARY_FIRST_2026;

    use super::*;

    pub fn init_vault(ctx: Context<InitVault>) -> Result<()> {
        let rent = Rent::get()?;
        let rent_exempt_amount = rent.minimum_balance(0);
        let owner = &ctx.accounts.owner;
        let vault = &ctx.accounts.vault;
        let system_program = &ctx.accounts.system_program;
        let owner_key = &ctx.accounts.owner.key();

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", owner_key.as_ref(), &[ctx.bumps.vault]]];

        create_account(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                CreateAccount {
                    from: owner.to_account_info(),
                    to: vault.to_account_info(),
                },
                signer_seeds,
            ),
            rent_exempt_amount,
            0,
            &system_program.key(),
        )
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        if vault.lamports() == 0 {
            msg!("Vault balance is zero");
        }

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            amount,
        )
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // withdrawals not available  after January first 2026 (1767218400)
        let current_timestamp = Clock::get()?.unix_timestamp;

        if current_timestamp >= JANUARY_FIRST_2026 {
            return Err(VaultErrors::WithdrawalNotAvailable.into());
        }

        let vault = &ctx.accounts.vault;
        let rent = Rent::get()?;
        let rent_exempt_amount = rent.minimum_balance(0);

        if vault.lamports() < rent_exempt_amount + amount {
            return Err(VaultErrors::InsufficientVaultFunds.into());
        }
        let owner_key = ctx.accounts.owner.key();
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", owner_key.as_ref(), &[ctx.bumps.vault]]];

        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.owner.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )
    }
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitVault<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultErrors {
    #[msg("Withdrawal will be available until January 1st 2026")]
    WithdrawalNotAvailable,
    #[msg("Withdrawal amount is greater than the available balance")]
    InsufficientVaultFunds,
}
