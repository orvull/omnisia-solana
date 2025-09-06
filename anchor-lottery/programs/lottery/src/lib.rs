use anchor_lang::prelude::*;
use anchor_lang::system_program;

// example program id generated for demonstration
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMQ5GvJNk6m");

#[program]
pub mod lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let lottery = &mut ctx.accounts.lottery;
        lottery.authority = *ctx.accounts.authority.key;
        lottery.tickets = Vec::new();
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {
        {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.player.to_account_info(),
                    to: ctx.accounts.lottery.to_account_info(),
                },
            );
            system_program::transfer(cpi_ctx, amount)?;
        }
        let lottery = &mut ctx.accounts.lottery;
        lottery.tickets.push(*ctx.accounts.player.key);
        Ok(())
    }

    pub fn draw(ctx: Context<Draw>) -> Result<()> {
        let total = ctx.accounts.lottery.tickets.len();
        require!(total > 0, LotteryError::NoPlayers);
        let slot = Clock::get()?.slot as usize;
        let winner_index = slot % total;
        let winner_key = ctx.accounts.lottery.tickets[winner_index];
        require!(winner_key == *ctx.accounts.winner.key, LotteryError::InvalidWinner);
        let amount = **ctx.accounts.lottery.to_account_info().lamports.borrow();
        **ctx.accounts.lottery.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.winner.to_account_info().try_borrow_mut_lamports()? += amount;
        let lottery = &mut ctx.accounts.lottery;
        lottery.tickets.clear();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + Lottery::MAX_SIZE)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub lottery: Account<'info, Lottery>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Draw<'info> {
    #[account(mut, has_one = authority)]
    pub lottery: Account<'info, Lottery>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub winner: AccountInfo<'info>,
}

#[account]
pub struct Lottery {
    pub authority: Pubkey,
    pub tickets: Vec<Pubkey>,
}

impl Lottery {
    pub const MAX_PLAYERS: usize = 64;
    pub const MAX_SIZE: usize = 32 + 4 + 32 * Self::MAX_PLAYERS;
}

#[error_code]
pub enum LotteryError {
    #[msg("No players in the lottery")]
    NoPlayers,
    #[msg("Winner account mismatch")]
    InvalidWinner,
}
