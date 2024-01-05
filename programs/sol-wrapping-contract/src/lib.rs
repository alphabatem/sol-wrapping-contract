pub mod errors;

use anchor_lang::prelude::*;
use anchor_lang::system_program::{self};
use anchor_spl::token_interface::{TokenInterface, Mint, TokenAccount};
use anchor_spl::associated_token::{self, AssociatedToken};
use anchor_spl::token;
use crate::errors::ErrorCode;

declare_id!("DX6quq5ypkFrGviQuMTX6KMLiEGVjnWaEXiDsVpNRP3e");

#[program]
pub mod sol_wrapping_contract {
    use super::*;

    pub fn wrap(ctx: Context<Wrap>, amount: u64) -> Result<()> {
        wrap_handler(ctx, amount)
    }

    pub fn unwrap(ctx: Context<Unwrap>) -> Result<()> {
        unwrap_handler(ctx)
    }
}


#[derive(Accounts)]
# [instruction (amount: u64)]
pub struct Wrap<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = native_mint,
        associated_token::authority = owner,
    )]
    pub wsol_account: Box<InterfaceAccount<'info, TokenAccount>>, 

    #[account(mut)]
    pub native_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(address = associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Unwrap<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = native_mint.key() == wsol_account.mint,
        constraint = owner.key() == wsol_account.owner,
    )]
    pub wsol_account: Box<InterfaceAccount<'info, TokenAccount>>, 

    #[account(mut)]
    pub native_mint: Box<InterfaceAccount<'info, Mint>>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn wrap_handler(
    ctx: Context<Wrap>,
    amount: u64
) -> Result<()> {

    // checks if ata already exists
    let wsol_amount: u64;
    let user_balance = ctx.accounts.wsol_account.amount;

    msg!("Desired WSOL amount in lamports: {:?}", amount);
    msg!("User WSOL balance in lamports: {:?}", user_balance);

    if user_balance >= amount {
        msg!("Enough WSOL in token account.");
        return Ok(())
    }

    wsol_amount = amount - user_balance;

    msg!("WSOL amount to wrap in lamports: {:?}", wsol_amount);

    if ctx.accounts.owner.lamports() < wsol_amount {
        return err!(ErrorCode::InsufficientBalance);
    }


    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.owner.to_account_info(),
                to: ctx.accounts.wsol_account.to_account_info(),
            },
        ),
        wsol_amount,
    )?;

    token::sync_native(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            token::SyncNative { 
                account: ctx.accounts.wsol_account.to_account_info() 
            }
        )
    )?;

    
    Ok(())
}


pub fn unwrap_handler(
    ctx: Context<Unwrap>,
) -> Result<()> {

    // check if token account exists
    let wsol_account_size = ctx.accounts.wsol_account.to_account_info().data_len();

    if wsol_account_size < 1 {
        return err!(ErrorCode::UninitializedTokenAccount);
    }

    token::close_account(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(), 
            token::CloseAccount { 
                account: ctx.accounts.wsol_account.to_account_info(),
                destination: ctx.accounts.owner.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(), 
            }
        )
    )?;
    msg!("Token Account Closed.");

    
    Ok(())
}
