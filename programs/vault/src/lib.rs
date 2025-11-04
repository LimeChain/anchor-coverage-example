#![allow(deprecated)]
#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

use anchor_lang::solana_program::entrypoint::ProgramResult;
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
        let _k = test_chained();
        let _c = test_chained2();
        let _a = test_question_mark(1);
        let _b = test_question_mark(2);
        test_conditional(1);
        test_conditional(2);
        if let Ok(v) = test_chained_errors() {
            msg!("vec: {:?}", v);
        }

        // withdrawals not available  after January first 2026 (1767218400)
        let current_timestamp = Clock::get()?.unix_timestamp;

        if current_timestamp >= JANUARY_FIRST_2026 {
            return Err(VaultErrors::WithdrawalNotAvailable.into());
        }

        let vault = &ctx.accounts.vault;
        let rent = Rent::get()?;
        let rent_exempt_amount = rent.minimum_balance(0);

        // msg!("vault.lamports: {}, rent_exempt_amount: {} + amount: {}", vault.lamports(), rent_exempt_amount, amount);
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

// #[inline(never)]
fn test_question_mark(a: i64) -> ProgramResult {
    let _s = "12345";
    test_question_mark_inner(a)?;
    Ok(())
}

// #[inline(never)]
fn test_question_mark_inner(a: i64) -> ProgramResult {
    if a == 1 {
        Err(ProgramError::InvalidArgument)
    } else {
        Err(ProgramError::Custom(a as _))
    }
}

// #[inline(never)]
fn test_chained() -> ProgramResult {
    let v = vec![1, 2, 3, 5];
    let _res = v
        .iter()
        .enumerate()
        .map(|e| e.1)
        .map(|e| *e)
        .filter(|e: &i32| *e > 4)
        .count();
    Ok(())
}

// #[inline(never)]
fn test_chained2() -> ProgramResult {
    let v = vec![1, 2, 3];
    let _res = v
        .iter()
        .enumerate()
        .map(|e| -> Option<i32> { Some(*e.1) })
        .map(|e| -> Option<i32> { Some(e?) })
        .filter(|e| e.unwrap() > 4)
        .count();
    Ok(())
}

// #[inline(never)]
fn test_conditional(a: i32) {
    if a == 1 {
        msg!("Got 1");
        msg!("Got 1");
        msg!("Got 1");
    } else {
        msg!("Got 2");
    }
}

// #[inline(never)]
fn test_chained_errors() -> std::result::Result<Vec<u8>, String> {
    let val = "1234";
    let p = val
        .parse::<i32>()
        .map_err(|e| e.to_string())?
        .try_to_vec()
        .map_err(|_e| "stop!".to_string())?;
    Ok(p)
}
