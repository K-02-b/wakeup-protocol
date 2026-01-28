use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("5nqDyKur7to6jwoMiRtJMUVnWB5ZApx3NVhdacFi5KRj");

const ADMIN_PUBKEY: Pubkey = pubkey!("C5vyBZ5RTCJU7rpicQ7BvQMnNoKLTu2HWApTdnL8i8Kh");
const TREASURY_PUBKEY: Pubkey = pubkey!("C5vyBZ5RTCJU7rpicQ7BvQMnNoKLTu2HWApTdnL8i8Kh");

#[program]
pub mod wakeup_protocol {
    use super::*;

    // 1. 用户质押：前端计算好 deadline 和 bucket (0-47)
    pub fn stake(ctx: Context<Stake>, amount: u64, deadline: i64, bucket: u8) -> Result<()> {
        // 第一步：先执行转账（此时不借用 user_state 的可变引用）
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.user_state.to_account_info(), // 这里是正常引用
            },
        );
        system_program::transfer(cpi_context, amount)?;

        // 第二步：转账完成后，再获取 user_state 的可变引用进行赋值
        let user_state = &mut ctx.accounts.user_state;
        user_state.owner = ctx.accounts.user.key();
        user_state.amount = amount;
        user_state.deadline = deadline;
        user_state.bucket = bucket;
        user_state.status = 1; // 1: Active

        msg!("Staked: {} for bucket: {}", amount, bucket);
        Ok(())
    }
    // 2. 服务器确认成功：返还质押金
    pub fn complete_checkin(ctx: Context<AdminAction>) -> Result<()> {
        let user_state = &mut ctx.accounts.user_state;
        require!(user_state.status == 1, WakeUpError::InvalidStatus);

        // 权限检查
        require_keys_eq!(ctx.accounts.admin.key(), ADMIN_PUBKEY, WakeUpError::Unauthorized);

        // 将本金从 PDA 退还给用户
        let amount = user_state.amount;
        **user_state.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.user_target.to_account_info().try_borrow_mut_lamports()? += amount;

        user_state.status = 2; // 2: Completed
        msg!("Check-in verified. Refunded: {}", amount);
        Ok(())
    }

    // 3. 服务器清算：将逾期者的钱转入国库
    pub fn liquidate(ctx: Context<AdminAction>) -> Result<()> {
        let clock = Clock::get()?;
        let user_state = &mut ctx.accounts.user_state;

        require_keys_eq!(ctx.accounts.admin.key(), ADMIN_PUBKEY, WakeUpError::Unauthorized);
        require!(user_state.status == 1, WakeUpError::InvalidStatus);
        require!(clock.unix_timestamp > user_state.deadline, WakeUpError::NotExpired);

        let amount = user_state.amount;
        **user_state.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.treasury.to_account_info().try_borrow_mut_lamports()? += amount;

        user_state.status = 3; // 3: Slashed
        msg!("Liquidation Success: User {} slashed {}", user_state.owner, amount);
        Ok(())
    }
}

#[account]
pub struct UserState {
    pub owner: Pubkey,   // 32
    pub amount: u64,     // 8
    pub deadline: i64,   // 8
    pub bucket: u8,      // 1
    pub status: u8,      // 1 (1:Active, 2:Done, 3:Slashed)
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        init, 
        payer = user, 
        space = 8 + 32 + 8 + 8 + 1 + 1,
        seeds = [b"wakeup", user.key().as_ref()], 
        bump
    )]
    pub user_state: Account<'info, UserState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(
        mut,
        seeds = [b"wakeup", user_target.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
    /// CHECK: 这里是接收退款的用户钱包
    #[account(mut)]
    pub user_target: UncheckedAccount<'info>,
    /// CHECK: 这里是国库钱包
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[error_code]
pub enum WakeUpError {
    #[msg("Unauthorized: Only Admin Server can call this.")]
    Unauthorized,
    #[msg("Challenge is not expired yet.")]
    NotExpired,
    #[msg("Invalid status for this action.")]
    InvalidStatus,
}