use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock, program_option::COption};
use anchor_spl::token::{self, TokenAccount, Token, Mint};
use std::convert::Into;
use std::convert::TryInto;

declare_id!("65fMP12hbWs8FrHxBcwYkjQwL2SVv43EisrMvUpWJdkN");

pub fn update_rewards_a(
    user: Option<&mut Box<Account<User>>>,
    pool: &mut Box<Account<Pool>>
) -> Result<()> {
    if let Some(u) = user {
        let clock = clock::Clock::get().unwrap();
        let last_time_reward_applicable = clock.unix_timestamp;
        u.reward_a = 0;
        u.reward_a_rate = u.balance_staked_a
                            .checked_mul(pool.apy_fee_a).unwrap()
                            .checked_div(100*365*24*60*60).unwrap();
        u.last_update_time_a = last_time_reward_applicable as u64;
    }
    Ok(())
}

pub fn update_rewards_a_claim(
    user: Option<&mut Box<Account<User>>>,
) -> Result<()> {
    if let Some(u) = user {
        let clock = clock::Clock::get().unwrap();
        let last_time_reward_applicable = clock.unix_timestamp;
        u.reward_a = u.reward_a.checked_add(
        ((last_time_reward_applicable as u64)
        .checked_sub(u.last_update_time_a as u64)
        .unwrap())
        .checked_mul(u.reward_a_rate as u64)
        .unwrap())
        .unwrap();
        u.last_update_time_a = last_time_reward_applicable as u64;
    }
    Ok(())
}

pub fn update_rewards_b(
    user: Option<&mut Box<Account<User>>>,
    pool: &mut Box<Account<Pool>>,
) -> Result<()> {
    if let Some(u) = user {
        let clock = clock::Clock::get().unwrap();
        let last_time_reward_applicable = clock.unix_timestamp;
        u.reward_b = 0;
        u.reward_b_rate = u.balance_staked_b
                            .checked_mul(pool.apy_fee_b).unwrap()
                            .checked_div(100*365*24*60*60).unwrap();
        u.last_update_time_b = last_time_reward_applicable as u64;
    }
    Ok(())
}

pub fn update_rewards_b_claim(
    user: Option<&mut Box<Account<User>>>,
) -> Result<()> {
    if let Some(u) = user {
        let clock = clock::Clock::get().unwrap();
        let last_time_reward_applicable = clock.unix_timestamp;
        u.reward_b = u.reward_b.checked_add(
        ((last_time_reward_applicable as u64)
        .checked_sub(u.last_update_time_b as u64)
        .unwrap())
        .checked_mul(u.reward_b_rate as u64)
        .unwrap())
        .unwrap();
        u.last_update_time_b = last_time_reward_applicable as u64;
    }
    Ok(())
}

pub fn update_rewards_c(
    user: Option<&mut Box<Account<User>>>,
    pool: &mut Box<Account<Pool>>
) -> Result<()> {
    if let Some(u) = user {
        let clock = clock::Clock::get().unwrap();
        let last_time_reward_applicable = clock.unix_timestamp;
            u.reward_c = 0;
            u.reward_c_rate = u.balance_staked_c
                                .checked_mul(pool.apy_fee_c).unwrap()
                                .checked_div(100*365*24*60*60).unwrap();
            u.last_update_time_c = last_time_reward_applicable as u64;
    }
    Ok(())
}

pub fn update_rewards_c_claim(
    user: Option<&mut Box<Account<User>>>,
) -> Result<()> {
    if let Some(u) = user {
        let clock = clock::Clock::get().unwrap();
        let last_time_reward_applicable = clock.unix_timestamp;
        u.reward_c = u.reward_c.checked_add(
        ((last_time_reward_applicable as u64)
        .checked_sub(u.last_update_time_c as u64)
        .unwrap())
        .checked_mul(u.reward_c_rate as u64)
        .unwrap())
        .unwrap();
        u.last_update_time_c = last_time_reward_applicable as u64;
    }
    Ok(())
}

#[program]
pub mod seeded_staking {
    use super::*;
    pub fn initialize(
        ctx: Context<Initialize>,
        pool_nonce: u8,
        ) -> ProgramResult {

        let pool = &mut ctx.accounts.pool;
        pool.authority = ctx.accounts.authority.key();
        pool.nonce = pool_nonce;
        pool.paused = false;
        pool.staking_mint = ctx.accounts.staking_mint.key();
        pool.staking_vault = ctx.accounts.staking_vault.key();
        pool.user_stake_count = 0;
        pool.staking_amount_a = 0;
        pool.staking_amount_b = 0;
        pool.staking_amount_c = 0;
        pool.admin_reward_amount = 0;
        pool.staking_duration_a = 14;
        pool.staking_duration_b = 30;
        pool.staking_duration_c = 60;

        pool.early_unstake_fee = 5;
        pool.unstake_fee = 5;

        pool.apy_fee_a = 45;
        pool.apy_fee_b = 75;
        pool.apy_fee_c = 120;
        Ok(())
    }

    pub fn create_user(ctx: Context<CreateUser>, nonce: u8) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.pool = *ctx.accounts.pool.to_account_info().key;
        user.owner = *ctx.accounts.owner.key;

        let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();

        user.stake_time_a = current_time;
        user.stake_time_b = current_time;
        user.stake_time_c = current_time;
        user.last_update_time_a = current_time;
        user.last_update_time_b = current_time;
        user.last_update_time_c = current_time;

        user.reward_a_rate = 0;
        user.reward_b_rate = 0;
        user.reward_c_rate = 0;
        user.reward_a = 0;
        user.reward_b = 0;
        user.reward_c = 0;

        user.balance_staked_a = 0;
        user.balance_staked_b = 0;
        user.balance_staked_c = 0;
        user.nonce = nonce;

        let pool = &mut ctx.accounts.pool;
        pool.user_stake_count = pool.user_stake_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn change_base_data(
        ctx: Context<ChangeBaseData>, 
        duration_a: u64, duration_b: u64, duration_c: u64,
        early_fee: u64, unstake_fee: u64,
        apy_a: u64, apy_b: u64, apy_c: u64,

    ) -> Result<()> {
        let pool = &mut ctx.accounts.pool;

        pool.staking_duration_a = duration_a;
        pool.staking_duration_b = duration_b;
        pool.staking_duration_c = duration_c;

        pool.early_unstake_fee = early_fee;
        pool.unstake_fee = unstake_fee;

        pool.apy_fee_a = apy_a;
        pool.apy_fee_b = apy_b;
        pool.apy_fee_c = apy_c;

        Ok(())
    }

    pub fn stake(ctx: Context<Stake>, amount: u64, staking_type:u64) -> Result<()> {
        if amount == 0 {
            return Err(ErrorCode::AmountMustBeGreaterThanZero.into());
        }

        let pool = &mut ctx.accounts.pool;
        
        if pool.paused {
            return Err(ErrorCode::PoolPaused.into());
        }
        let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
        if staking_type == pool.staking_duration_a {
            ctx.accounts.user.balance_staked_a = ctx.accounts.user.balance_staked_a.checked_add(amount).unwrap();
            ctx.accounts.user.stake_time_a = current_time;
            pool.staking_amount_a = pool.staking_amount_a.checked_add(amount).unwrap();

            let _user_obj = Some(&mut ctx.accounts.user);
            update_rewards_a( _user_obj, pool ).unwrap();
        }
        if staking_type == pool.staking_duration_b {
            ctx.accounts.user.balance_staked_b = ctx.accounts.user.balance_staked_b.checked_add(amount).unwrap();
            ctx.accounts.user.stake_time_b = current_time;
            pool.staking_amount_b = pool.staking_amount_b.checked_add(amount).unwrap();

            let _user_obj = Some(&mut ctx.accounts.user);
            update_rewards_b( _user_obj, pool ).unwrap();
        }
        if staking_type == pool.staking_duration_c {
            ctx.accounts.user.balance_staked_c = ctx.accounts.user.balance_staked_c.checked_add(amount).unwrap();
            ctx.accounts.user.stake_time_c = current_time;
            pool.staking_amount_c = pool.staking_amount_c.checked_add(amount).unwrap();
            
            let _user_obj = Some(&mut ctx.accounts.user);
            update_rewards_c( _user_obj, pool ).unwrap();
        }

        // Transfer tokens into the stake vault.
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.stake_from_account.to_account_info(),
                    to: ctx.accounts.staking_vault.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(), //todo use user account as signer
                },
            );
            token::transfer(cpi_ctx, amount)?;
        }

        Ok(())
    }


    pub fn unstake(ctx: Context<Stake>, spt_amount: u64, staking_type:u64) -> Result<()> {
        let mut unstake_amount = spt_amount;

        let pool = &mut ctx.accounts.pool;
        
        if spt_amount == 0 {
            return Err(ErrorCode::AmountMustBeGreaterThanZero.into());
        }
        
        if staking_type == pool.staking_duration_a {
            if ctx.accounts.user.balance_staked_a < spt_amount {
                return Err(ErrorCode::InsufficientFundUnstake.into());
            }
            let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
            if current_time < ctx.accounts.user.stake_time_a.checked_add(pool.staking_duration_a+120).unwrap() {
                if ctx.accounts.user.balance_staked_a < spt_amount {
                    return Err(ErrorCode::InsufficientFundUnstake.into());
                }
                ctx.accounts.user.balance_staked_a= ctx.accounts.user.balance_staked_a.checked_sub(spt_amount).unwrap();
                pool.staking_amount_a = pool.staking_amount_a.checked_sub(spt_amount).unwrap();
                pool.admin_reward_amount = pool.admin_reward_amount.checked_add((spt_amount.checked_mul(pool.early_unstake_fee as u64).unwrap()).checked_div(100 as u64).unwrap()).unwrap();
                unstake_amount =(unstake_amount.checked_mul(100-pool.early_unstake_fee).unwrap()).checked_div(100).unwrap();
                ctx.accounts.user.stake_time_a = current_time;
                
                let _user_obj = Some( &mut ctx.accounts.user );
                update_rewards_a( _user_obj, pool ).unwrap();
            }
            else{
                let user_obj = Some(&mut ctx.accounts.user);
                update_rewards_a_claim(
                    user_obj,
                )
                .unwrap();
                ctx.accounts.user.balance_staked_a = ctx.accounts.user.balance_staked_a.checked_sub(spt_amount).unwrap();
                pool.staking_amount_a = pool.staking_amount_a.checked_sub(spt_amount).unwrap();
                unstake_amount = unstake_amount.checked_add(ctx.accounts.user.reward_a).unwrap().checked_mul(100-pool.unstake_fee as u64).unwrap().checked_div(100 as u64).unwrap();
                pool.admin_reward_amount = pool.admin_reward_amount.checked_add(unstake_amount.checked_add(ctx.accounts.user.reward_a).unwrap().checked_mul(pool.unstake_fee as u64).unwrap().checked_div(100 as u64).unwrap()).unwrap();

                ctx.accounts.user.reward_a = 0;
                ctx.accounts.user.stake_time_a = current_time;

                let _user_obj = Some( &mut ctx.accounts.user );
                update_rewards_a( _user_obj, pool ).unwrap();
            }
        }
        if staking_type == pool.staking_duration_b {
            if ctx.accounts.user.balance_staked_b < spt_amount {
                return Err(ErrorCode::InsufficientFundUnstake.into());
            }
            let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
            if current_time < ctx.accounts.user.stake_time_b.checked_add(pool.staking_duration_b+120).unwrap() {
                if ctx.accounts.user.balance_staked_b < spt_amount {
                    return Err(ErrorCode::InsufficientFundUnstake.into());
                }
                ctx.accounts.user.balance_staked_b= ctx.accounts.user.balance_staked_b.checked_sub(spt_amount).unwrap();
                pool.staking_amount_b = pool.staking_amount_b.checked_sub(spt_amount).unwrap();
                pool.admin_reward_amount = pool.admin_reward_amount.checked_add((spt_amount.checked_mul(pool.early_unstake_fee as u64).unwrap()).checked_div(100 as u64).unwrap()).unwrap();
                unstake_amount =(unstake_amount.checked_mul(100-pool.early_unstake_fee).unwrap()).checked_div(100).unwrap();
                ctx.accounts.user.stake_time_b = current_time;

                let _user_obj = Some(&mut ctx.accounts.user);
                update_rewards_b( _user_obj, pool ).unwrap();

            }
            else{
                let user_obj = Some(&mut ctx.accounts.user);
                update_rewards_b_claim(
                    user_obj,
                )
                .unwrap();
                ctx.accounts.user.balance_staked_b = ctx.accounts.user.balance_staked_b.checked_sub(spt_amount).unwrap();
                pool.staking_amount_b = pool.staking_amount_b.checked_sub(spt_amount).unwrap();
                // unstake_amount = unstake_amount.checked_add(ctx.accounts.user.reward_b).unwrap();
                unstake_amount = unstake_amount.checked_add(ctx.accounts.user.reward_b).unwrap().checked_mul(100-pool.unstake_fee as u64).unwrap().checked_div(100 as u64).unwrap();
                pool.admin_reward_amount = pool.admin_reward_amount.checked_add(unstake_amount.checked_add(ctx.accounts.user.reward_b).unwrap().checked_mul(pool.unstake_fee as u64).unwrap().checked_div(100 as u64).unwrap()).unwrap();


                ctx.accounts.user.reward_b = 0;
                ctx.accounts.user.stake_time_b = current_time;

                let _user_obj = Some(&mut ctx.accounts.user);
                update_rewards_b( _user_obj, pool ).unwrap();
            }
            
        }
        if staking_type == pool.staking_duration_c {
            if ctx.accounts.user.balance_staked_c < spt_amount {
                return Err(ErrorCode::InsufficientFundUnstake.into());
            }
            let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
            if current_time < ctx.accounts.user.stake_time_c.checked_add(pool.staking_duration_c+120).unwrap() {
                if ctx.accounts.user.balance_staked_c < spt_amount {
                    return Err(ErrorCode::InsufficientFundUnstake.into());
                }
                ctx.accounts.user.balance_staked_c= ctx.accounts.user.balance_staked_c.checked_sub(spt_amount).unwrap();
                pool.staking_amount_c = pool.staking_amount_c.checked_sub(spt_amount).unwrap();
                pool.admin_reward_amount = pool.admin_reward_amount.checked_add((spt_amount.checked_mul(pool.early_unstake_fee as u64).unwrap()).checked_div(100 as u64).unwrap()).unwrap();
                unstake_amount =(unstake_amount.checked_mul(100-pool.early_unstake_fee).unwrap()).checked_div(100).unwrap();
                ctx.accounts.user.stake_time_c = current_time;

                let _user_obj = Some(&mut ctx.accounts.user);
                update_rewards_b( _user_obj, pool ).unwrap();
                // return Err(ErrorCode::NotEnoughUnstakePeriod.into());
            }
            else{
                let user_obj = Some(&mut ctx.accounts.user);
                update_rewards_c_claim( user_obj ).unwrap();

                ctx.accounts.user.balance_staked_c = ctx.accounts.user.balance_staked_c.checked_sub(spt_amount).unwrap();
                pool.staking_amount_c = pool.staking_amount_c.checked_sub(spt_amount).unwrap();
                // unstake_amount = unstake_amount.checked_add(ctx.accounts.user.reward_c).unwrap();
                unstake_amount = unstake_amount.checked_add(ctx.accounts.user.reward_c).unwrap().checked_mul(100-pool.unstake_fee as u64).unwrap().checked_div(100 as u64).unwrap();
                pool.admin_reward_amount = pool.admin_reward_amount.checked_add(unstake_amount.checked_add(ctx.accounts.user.reward_c).unwrap().checked_mul(pool.unstake_fee as u64).unwrap().checked_div(100 as u64).unwrap()).unwrap();

                ctx.accounts.user.reward_c = 0;
                ctx.accounts.user.stake_time_c = current_time;

                let _user_obj = Some(&mut ctx.accounts.user);
                update_rewards_b( _user_obj, pool ).unwrap();
            }
        }

        {
            let seeds = &[
                ctx.accounts.pool.to_account_info().key.as_ref(),
                &[ctx.accounts.pool.nonce],
            ];
            let pool_signer = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.staking_vault.to_account_info(),
                    to: ctx.accounts.stake_from_account.to_account_info(),
                    authority: ctx.accounts.pool_signer.to_account_info(),
                },
                pool_signer,
            );
            let vault_balance = ctx.accounts.staking_vault.amount;
            let mut real_amount = unstake_amount;
            if vault_balance < unstake_amount{
                real_amount = vault_balance; 
            }
            token::transfer(cpi_ctx, real_amount.try_into().unwrap())?;
        }

        Ok(())
    }

    pub fn authorize_funder(ctx: Context<FunderChange>, funder_to_add: Pubkey) -> Result<()> {
        if funder_to_add == ctx.accounts.pool.authority.key() {
            return Err(ErrorCode::FunderAlreadyAuthorized.into());
        }
        let funders = &mut ctx.accounts.pool.funders;
        if funders.iter().any(|x| *x == funder_to_add) {
            return Err(ErrorCode::FunderAlreadyAuthorized.into());
        }
        let default_pubkey = Pubkey::default();
        if let Some(idx) = funders.iter().position(|x| *x == default_pubkey) {
            funders[idx] = funder_to_add;
        } else {
            return Err(ErrorCode::MaxFunders.into());
        }
        Ok(())
    }

    pub fn deauthorize_funder(ctx: Context<FunderChange>, funder_to_remove: Pubkey) -> Result<()> {
        if funder_to_remove == ctx.accounts.pool.authority.key() {
            return Err(ErrorCode::CannotDeauthorizePoolAuthority.into());
        }
        let funders = &mut ctx.accounts.pool.funders;
        if let Some(idx) = funders.iter().position(|x| *x == funder_to_remove) {
            funders[idx] = Pubkey::default();
        } else {
            return Err(ErrorCode::CannotDeauthorizeMissingAuthority.into());
        }
        Ok(())
    }

    pub fn fund_staking(ctx: Context<FundStaking>, amount: u64) -> Result<()> {

        if amount > 0 {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.from.to_account_info(),
                    to: ctx.accounts.staking_vault.to_account_info(),
                    authority: ctx.accounts.funder.to_account_info(),
                },
            );
            ctx.accounts.pool.admin_reward_amount = ctx.accounts.pool.admin_reward_amount.checked_add(amount as u64).unwrap();
            token::transfer(cpi_ctx, amount)?;
        }

        Ok(())
    }

    pub fn withdraw(ctx: Context<Fund>, amount: u64) -> Result<()> {

        if amount > 0 {
            if amount <= ctx.accounts.pool.admin_reward_amount {
                let seeds = &[
                    ctx.accounts.pool.to_account_info().key.as_ref(),
                    &[ctx.accounts.pool.nonce],
                ];
                let pool_signer = &[&seeds[..]];
    
                let cpi_ctx = CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    token::Transfer {
                        from: ctx.accounts.staking_vault.to_account_info(),
                        to: ctx.accounts.from.to_account_info(),
                        authority: ctx.accounts.pool_signer.to_account_info(),
                    },
                    pool_signer,
                );
                ctx.accounts.pool.admin_reward_amount = ctx.accounts.pool.admin_reward_amount.checked_sub(amount as u64 ).unwrap();
                token::transfer(cpi_ctx, amount)?;
            }
            else{
                return Err(ErrorCode::CannotDeauthorizeMissingAuthority.into());

            }
           
        }

        Ok(())
    }

    pub fn claim(ctx: Context<ClaimReward>, staking_type:u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        if staking_type == pool.staking_duration_a {
            let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
            if current_time > ctx.accounts.user.stake_time_a.checked_add(pool.staking_duration_a+120).unwrap() {
                let user_opt1 = Some(&mut ctx.accounts.user);
                update_rewards_a_claim(
                    user_opt1,
                )
                .unwrap();
        
                let seeds = &[
                    pool.to_account_info().key.as_ref(),
                    &[pool.nonce],
                ];
                let pool_signer = &[&seeds[..]];
                
                if ctx.accounts.user.reward_a > 0 {
                    let mut reward_amount = ctx.accounts.user.reward_a;
                    let vault_balance = ctx.accounts.staking_vault.amount;
        
                    ctx.accounts.user.reward_a = 0;
                    if vault_balance < reward_amount {
                        reward_amount = vault_balance;
                    }
        
                    if reward_amount > 0 {
                        let cpi_ctx = CpiContext::new_with_signer(
                            ctx.accounts.token_program.to_account_info(),
                            token::Transfer {
                                from: ctx.accounts.staking_vault.to_account_info(),
                                to: ctx.accounts.reward_a_account.to_account_info(),
                                authority: ctx.accounts.pool_signer.to_account_info(),
                            },
                            pool_signer,
                        );
                        token::transfer(cpi_ctx, reward_amount)?;
                    }
                }
            }
            else{
                return Err(ErrorCode::CannotDeauthorizeMissingAuthority.into());
            }
        }
        
        if staking_type == pool.staking_duration_b {
            let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
            if current_time > ctx.accounts.user.stake_time_b.checked_add(pool.staking_duration_b+120).unwrap() {
                let user_opt1 = Some(&mut ctx.accounts.user);
                update_rewards_b_claim(
                    user_opt1,
                )
                .unwrap();
        
                let seeds = &[
                    pool.to_account_info().key.as_ref(),
                    &[pool.nonce],
                ];
                let pool_signer = &[&seeds[..]];
                
                if ctx.accounts.user.reward_b > 0 {
                    let mut reward_amount = ctx.accounts.user.reward_b;
                    let vault_balance = ctx.accounts.staking_vault.amount;
        
                    ctx.accounts.user.reward_b = 0;
                    if vault_balance < reward_amount {
                        reward_amount = vault_balance;
                    }
        
                    if reward_amount > 0 {
                        let cpi_ctx = CpiContext::new_with_signer(
                            ctx.accounts.token_program.to_account_info(),
                            token::Transfer {
                                from: ctx.accounts.staking_vault.to_account_info(),
                                to: ctx.accounts.reward_a_account.to_account_info(),
                                authority: ctx.accounts.pool_signer.to_account_info(),
                            },
                            pool_signer,
                        );
                        token::transfer(cpi_ctx, reward_amount)?;
                    }
                }
            }
            else{
                return Err(ErrorCode::CannotDeauthorizeMissingAuthority.into());
            }
        }
        
        if staking_type == pool.staking_duration_c {
            let current_time: u64 = clock::Clock::get().unwrap().unix_timestamp.try_into().unwrap();
            if current_time > ctx.accounts.user.stake_time_c.checked_add(pool.staking_duration_c+120).unwrap() {
                let user_opt2 = Some(&mut ctx.accounts.user);
                update_rewards_c_claim(
                    user_opt2,
                )
                .unwrap();
        
                let seeds = &[
                    pool.to_account_info().key.as_ref(),
                    &[pool.nonce],
                ];
                let pool_signer = &[&seeds[..]];
                if ctx.accounts.user.reward_c > 0 {
                    let mut reward_amount = ctx.accounts.user.reward_c;
                    let vault_balance = ctx.accounts.staking_vault.amount;
        
                    ctx.accounts.user.reward_c = 0;
                    if vault_balance < reward_amount {
                        reward_amount = vault_balance;
                    }
        
                    if reward_amount > 0 {
                        let cpi_ctx = CpiContext::new_with_signer(
                            ctx.accounts.token_program.to_account_info(),
                            token::Transfer {
                                from: ctx.accounts.staking_vault.to_account_info(),
                                to: ctx.accounts.reward_a_account.to_account_info(),
                                authority: ctx.accounts.pool_signer.to_account_info(),
                            },
                            pool_signer,
                        );
                        token::transfer(cpi_ctx, reward_amount)?;
                    }
                }
            }
            else{
                return Err(ErrorCode::CannotDeauthorizeMissingAuthority.into());
            }
        }
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(pool_nonce: u8)]
pub struct Initialize<'info> {
    authority: UncheckedAccount<'info>,

    staking_mint: Box<Account<'info, Mint>>,
    #[account(
        constraint = staking_vault.mint == staking_mint.key(),
        constraint = staking_vault.owner == pool_signer.key(),
        //strangely, spl maintains this on owner reassignment for non-native accounts
        //we don't want to be given an account that someone else could close when empty
        //because in our "pool close" operation we want to assert it is still open
        constraint = staking_vault.close_authority == COption::None,
    )]
    staking_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [
            pool.to_account_info().key.as_ref()
        ],
        bump = pool_nonce,
    )]
    pool_signer: UncheckedAccount<'info>,

    #[account(
        zero,
    )]
    pool: Box<Account<'info, Pool>>,
    
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(nonce: u8)]
pub struct CreateUser<'info> {
    // Stake instance.
    #[account(
        mut,
        constraint = !pool.paused,
    )]
    pool: Box<Account<'info, Pool>>,
    // Member.
    #[account(
        init,
        payer = owner,
        seeds = [
            owner.key.as_ref(), 
            pool.to_account_info().key.as_ref()
        ],
        bump = nonce,
    )]
    user: Box<Account<'info, User>>,
    owner: Signer<'info>,
    // Misc.
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ChangeBaseData<'info> {
    // Global accounts for the staking instance.
    #[account(
        mut, 
        has_one = authority,
    )]
    pool: Box<Account<'info, Pool>>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Stake<'info> {
    // Global accounts for the staking instance.
    #[account(
        mut, 
        has_one = staking_vault,
    )]
    pool: Box<Account<'info, Pool>>,
    #[account(
        mut,
        constraint = staking_vault.owner == *pool_signer.key,
    )]
    staking_vault: Box<Account<'info, TokenAccount>>,

    // User.
    #[account(
        mut, 
        has_one = owner, 
        has_one = pool,
        seeds = [
            owner.key.as_ref(), 
            pool.to_account_info().key.as_ref()
        ],
        bump = user.nonce,
    )]
    user: Box<Account<'info, User>>,
    owner: Signer<'info>,
    #[account(mut)]
    stake_from_account: Box<Account<'info, TokenAccount>>,

    // Program signers.
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref()
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,

    // Misc.
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FunderChange<'info> {
    // Global accounts for the staking instance.
    #[account(
        mut, 
        has_one = authority,
    )]
    pool: Box<Account<'info, Pool>>,
    authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Fund<'info> {
    // Global accounts for the staking instance.
    #[account(
        mut,
        has_one = staking_vault,
        constraint = !pool.paused,
    )]
    pool: Box<Account<'info, Pool>>,
    #[account(mut)]
    staking_vault: Box<Account<'info, TokenAccount>>,
    #[account(
    //     //require signed funder auth - otherwise constant micro fund could hold funds hostage
        constraint = funder.key() == pool.authority || pool.funders.iter().any(|x| *x == funder.key()),
    )]
    funder: Signer<'info>,
    #[account(mut)]
    from: Box<Account<'info, TokenAccount>>,
    
    // Program signers.
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref()
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,

    // Misc.
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct FundStaking<'info> {
    // Global accounts for the staking instance.
    #[account(
        mut, 
        has_one = staking_vault,
        constraint = !pool.paused,
    )]
    pool: Box<Account<'info, Pool>>,
    #[account(mut)]
    staking_vault: Box<Account<'info, TokenAccount>>,
    #[account(mut
        //require signed funder auth - otherwise constant micro fund could hold funds hostage
        // constraint = funder.key() == pool.authority || pool.funders.iter().any(|x| *x == funder.key()),
        // constraint = !pool.paused,
    )]
    funder: Signer<'info>,
    #[account(mut)]
    from: Box<Account<'info, TokenAccount>>,
    
    // Program signers.
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref()
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,

    // Misc.
    token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    // Global accounts for the staking instance.
    #[account(
        mut, 
        has_one = staking_vault,
    )]
    pool: Box<Account<'info, Pool>>,
    #[account(mut)]
    staking_vault: Box<Account<'info, TokenAccount>>,
    // User.
    #[account(
        mut,
        has_one = owner,
        has_one = pool,
        seeds = [
            owner.to_account_info().key.as_ref(),
            pool.to_account_info().key.as_ref()
        ],
        bump = user.nonce,
    )]
    user: Box<Account<'info, User>>,
    owner: Signer<'info>,
    #[account(mut)]
    reward_a_account:Box<Account<'info, TokenAccount>>,
    // Program signers.
    #[account(
        seeds = [
            pool.to_account_info().key.as_ref()
        ],
        bump = pool.nonce,
    )]
    pool_signer: UncheckedAccount<'info>,

    // Misc.
    token_program: Program<'info, Token>,
}

#[account]
pub struct Pool {
    /// Priviledged account.
    pub authority: Pubkey,
    /// Nonce to derive the program-derived address owning the vaults.
    pub nonce: u8,
    /// Paused state of the program
    pub paused: bool,
    /// Mint of the token that can be staked.
    pub staking_mint: Pubkey,
    /// Vault to store staked tokens.
    pub staking_vault: Pubkey,
    /// Users staked
    pub user_stake_count: u32,
    /// authorized funders
    /// [] because short size, fixed account size, and ease of use on 
    /// client due to auto generated account size property
    pub funders: [Pubkey; 5],
    /// admin reward amount
    pub staking_amount_a: u64,
    pub staking_amount_b: u64,
    pub staking_amount_c: u64,
    pub admin_reward_amount : u64,

    pub staking_duration_a: u64,
    pub staking_duration_b: u64,
    pub staking_duration_c: u64,

    pub early_unstake_fee: u64,
    pub unstake_fee: u64,

    pub apy_fee_a: u64,
    pub apy_fee_b: u64,
    pub apy_fee_c: u64,

}

#[account]
#[derive(Default)]
pub struct User {
    /// Pool the this user belongs to.
    pub pool: Pubkey,
    /// The owner of this account.
    pub owner: Pubkey,
    /// The amount of token claimed(7days).
    pub reward_a: u64,
    /// The amount of token claimed(14days).
    pub reward_b: u64,
    /// The amount of token claimed(30days)
    pub reward_c : u64,
    /// The amount of token A pending claim.
    pub reward_a_rate: u64,
    /// The amount of token B pending claim.
    pub reward_b_rate: u64,
    /// The amount of token C pending claim.
    pub reward_c_rate: u64,
    /// The amount staked(7days).
    pub balance_staked_a: u64,
    /// The amount staked(14days).
    pub balance_staked_b: u64,
    /// The amount staked(30days).
    pub balance_staked_c: u64,
    /// The last time reward states were updated(7days).
    pub last_update_time_a: u64,
    /// The last time reward states were updated(14days).
    pub last_update_time_b: u64,
    /// The last time reward states were updated(30days).
    pub last_update_time_c: u64,
    /// The last stake time(7days)
    pub stake_time_a: u64,
    /// The last stake time(14days)
    pub stake_time_b: u64,
    /// The last stake time(30days)
    pub stake_time_c: u64,
    /// Signer nonce.
    pub nonce: u8,
    /// Staking type
    pub staking_type: u64,
}

#[error]
pub enum ErrorCode {
    #[msg("Insufficient funds to unstake.")]
    InsufficientFundUnstake,
    #[msg("Amount must be greater than zero.")]
    AmountMustBeGreaterThanZero,
    #[msg("Reward B cannot be funded - pool is single stake.")]
    SingleStakeTokenBCannotBeFunded,
    #[msg("Pool is paused.")]
    PoolPaused,
    #[msg("Duration cannot be shorter than one day.")]
    DurationTooShort,
    #[msg("Provided funder is already authorized to fund.")]
    FunderAlreadyAuthorized,
    #[msg("Maximum funders already authorized.")]
    MaxFunders,
    #[msg("Cannot deauthorize the primary pool authority.")]
    CannotDeauthorizePoolAuthority,
    #[msg("Authority not found for deauthorization.")]
    CannotDeauthorizeMissingAuthority,
    #[msg("Can unstake only after 30 days more.")]
    NotEnoughUnstakePeriod,
}
 