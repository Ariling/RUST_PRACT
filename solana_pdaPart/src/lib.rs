use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HelloState {
    is_initialized: bool,
}

// 필요한 계정들
/// 1. [signer, writable] 자금 제공 계정
/// 2. [writable] PDA 계정
/// 3. [] 시스템 프로그램
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    const ACCOUNT_DATA_LEN: usize = std::mem::size_of::<HelloState>();

    let accounts_iter = &mut accounts.iter();
    // 필요한 계정들 가져오기
    let funding_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // PDA와 범프 찾기
    let (pda, bump) = Pubkey::find_program_address(
        &[b"customaddress", funding_account.key.as_ref()],
        program_id
    );

    if pda.ne(pda_account.key) {
        return Err(ProgramError::InvalidAccountData);
    }

    // 필요한 lamports 평가 및 트랜잭션 명령어 생성
    let lamports_required = Rent::get()?.minimum_balance(ACCOUNT_DATA_LEN);
    let create_pda_account_ix = system_instruction::create_account(
        funding_account.key,
        pda_account.key,
        lamports_required,
        ACCOUNT_DATA_LEN.try_into().unwrap(),
        program_id,
    );
    
    // PDA를 추가 서명자로 하여 명령어 호출
    invoke_signed(
        &create_pda_account_ix,
        &[
            funding_account.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[b"customaddress", funding_account.key.as_ref(), &[bump]]],
    )?;

    // PDA의 상태 설정
    let mut pda_account_state = HelloState {
        is_initialized: true,
    };
    pda_account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;

    Ok(())
}