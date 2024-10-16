use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError,
};

// 임시 프로그램 ID 선언 -> 에서 실제로 완성된 부분으로 바꿈
solana_program::declare_id!("EcMNnUTzAaRJwHE26N7uFDV2nXVNkMjwCmzdXtxDvpbi");

// 커스텀 에러 정의
#[derive(Debug)]
enum CustomError {
    InsufficientFundsForTransaction,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

/// Transfers lamports from one account (must be program owned)
/// to another account. The recipient can be any account
fn transfer_service_fee_lamports(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount_of_lamports: u64,
) -> ProgramResult {
    // Does the from account have enough lamports to transfer?
    if **from_account.try_borrow_lamports()? < amount_of_lamports {
        return Err(CustomError::InsufficientFundsForTransaction.into());
    }
    // Debit from_account and credit to_account
    **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    Ok(())
}

/// Primary function handler associated with instruction sent
/// to your program
fn instruction_handler(accounts: &[AccountInfo]) -> ProgramResult {
    // Get the 'from' and 'to' accounts
    let account_info_iter = &mut accounts.iter();
    let from_account = next_account_info(account_info_iter)?;
    let to_service_account = next_account_info(account_info_iter)?;

    // Extract a service 'fee' of 5 lamports for performing this instruction
    transfer_service_fee_lamports(from_account, to_service_account, 5u64)?;

    // Perform the primary instruction
    msg!("Service fee transferred successfully");
    // ... 여기에 추가적인 명령어 처리 로직을 구현할 수 있습니다.

    Ok(())
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Solana transfer program entrypoint");

    // Call the instruction handler
    instruction_handler(accounts)
}