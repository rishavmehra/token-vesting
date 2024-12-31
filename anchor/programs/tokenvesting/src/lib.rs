#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
  Mint, TokenInterface, TokenAccount
};

declare_id!("coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF");

pub const ANCHOR_DISCRIMINATOR: usize = 8;

#[program]
pub mod tokenvesting {
  use super::*;

  pub fn create_vesting_account(
    ctx: Context<CreateVestingAccount>,
    company_name: String,    
    
  )-> Result<()> {
    *ctx.accounts.vesting_account = VestingAccount{
      owner: ctx.accounts.signer.key(),
      mint: ctx.accounts.mint.key(),
      treasury_token_account: ctx.accounts.treasury_token_account.key(),
      company_name,
      treasury_bump: ctx.bumps.treasury_token_account,
      bump: ctx.bumps.vesting_account
    };
    Ok(())
  }

  pub fn create_employee_account(
    ctx: Context<CreateEmployeeAccount>,
    start_time: i64,
    end_time: i64,
    token_amount:i64,
    cliff_time: i64,
  ) -> Result<()>{
    *ctx.accounts.employee_account = EmployeeAccount{
      beneficiary: ctx.accounts.beneficiary.key(),
      start_time,
      end_time,
      token_amount,
      cliff_time,
      token_withdraw:0,
      vesting_account: ctx.accounts.vesting_account.key(),
      bump: ctx.bumps.employee_account
    };
    Ok(())
  }

}

#[derive(Accounts)]
#[instruction[company_name: String]]
pub struct CreateVestingAccount<'info> {
  #[account(mut)]
  pub signer: Signer<'info>,
  
  #[account(
    init, 
    payer = signer,
    space = ANCHOR_DISCRIMINATOR + VestingAccount::INIT_SPACE,
    seeds = [company_name.as_ref()],
    bump
  )]
  pub vesting_account: Account<'info, VestingAccount>,
  pub mint: InterfaceAccount<'info, Mint>,

  #[account(
    init,
    token::mint = mint,
    token::authority = treasury_token_account, 
    payer = signer,
    seeds = [b"vesting_treasury", company_name.as_bytes()],
    bump
  )]
  pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,  

  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateEmployeeAccount<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,
  pub beneficiary: SystemAccount<'info>,
  #[account(has_one=owner)]
  pub vesting_account: Account<'info, VestingAccount>,
  #[account(
    init,
    space = ANCHOR_DISCRIMINATOR+EmployeeAccount::INIT_SPACE,
    payer = owner,
    seeds = [b"employee_vesting", beneficiary.key.as_ref(), vesting_account.key().as_ref()],
    bump
  )]
  pub employee_account: Account<'info, EmployeeAccount>,
  pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
pub struct VestingAccount{
  pub owner: Pubkey,
  pub mint: Pubkey,
  pub treasury_token_account: Pubkey,
  #[max_len(30)]
  pub company_name: String,
  pub treasury_bump: u8,
  pub bump: u8
}

#[account]
#[derive(InitSpace)]
pub struct EmployeeAccount {
  pub beneficiary: Pubkey,
  pub start_time: i64,
  pub end_time: i64,
  pub token_amount: i64,
  pub token_withdraw: i64,
  pub cliff_time: i64,
  pub vesting_account: Pubkey,
  pub bump: u8
}
