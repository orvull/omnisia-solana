use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{clock::Clock, Sysvar},
};

#[derive(BorshSerialize, BorshDeserialize, Debug, Default)]
pub struct Lottery {
    pub authority: Pubkey,
    pub tickets: Vec<Pubkey>,
}

pub enum LotteryInstruction {
    Initialize,
    BuyTicket,
    Draw,
}

impl LotteryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.first() {
            Some(0) => Ok(Self::Initialize),
            Some(1) => Ok(Self::BuyTicket),
            Some(2) => Ok(Self::Draw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = LotteryInstruction::unpack(instruction_data)?;
    match instruction {
        LotteryInstruction::Initialize => initialize(accounts, program_id),
        LotteryInstruction::BuyTicket => buy_ticket(accounts),
        LotteryInstruction::Draw => draw(accounts),
    }
}

fn initialize(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let lottery_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    if lottery_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    let mut lottery = Lottery::default();
    lottery.authority = *authority.key;
    lottery.serialize(&mut &mut lottery_account.try_borrow_mut_data()?[..])?;
    Ok(())
}

fn buy_ticket(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let lottery_account = next_account_info(accounts_iter)?;
    let player = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let mut lottery = Lottery::try_from_slice(&lottery_account.try_borrow_data()?)?;
    invoke(
        &system_instruction::transfer(player.key, lottery_account.key, 1_000_000),
        &[player.clone(), lottery_account.clone(), system_program.clone()],
    )?;
    lottery.tickets.push(*player.key);
    lottery.serialize(&mut &mut lottery_account.try_borrow_mut_data()?[..])?;
    Ok(())
}

fn draw(accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let lottery_account = next_account_info(accounts_iter)?;
    let authority = next_account_info(accounts_iter)?;
    let winner_account = next_account_info(accounts_iter)?;
    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut lottery = Lottery::try_from_slice(&lottery_account.try_borrow_data()?)?;
    if lottery.tickets.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let slot = Clock::get()?.slot as usize;
    let idx = slot % lottery.tickets.len();
    if *winner_account.key != lottery.tickets[idx] {
        return Err(ProgramError::InvalidAccountData);
    }
    let pot = **lottery_account.try_borrow_lamports()?;
    **lottery_account.try_borrow_mut_lamports()? -= pot;
    **winner_account.try_borrow_mut_lamports()? += pot;
    lottery.tickets.clear();
    lottery.serialize(&mut &mut lottery_account.try_borrow_mut_data()?[..])?;
    Ok(())
}
