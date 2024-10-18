// use solana_program::{
//     account_info::next_account_info, account_info::AccountInfo, entrypoint,
//     entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, system_instruction, sysvar::{rent::Rent, Sysvar}
// };
 
// entrypoint!(process_instruction);
 
// fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     let account_info_iter = &mut accounts.iter();
 
//     // let payer_account_info = next_account_info(account_info_iter)?;
//     // let pda_account_info = next_account_info(account_info_iter)?;
//     // let rent_sysvar_account_info = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

//     // creation code Part
 
//     // find space and minimum rent required for account
//     // let space = instruction_data[0];
//     // let bump = instruction_data[1];
//     // let rent_lamports = rent_sysvar_account_info.minimum_balance(space.into());
 
//     // invoke_signed(
//     //     &system_instruction::create_account(
//     //         &payer_account_info.key,
//     //         &pda_account_info.key,
//     //         rent_lamports,
//     //         space.into(),
//     //         program_id
//     //     ),
//     //     &[
//     //         payer_account_info.clone(),
//     //         pda_account_info.clone()
//     //     ],
//     //     &[&[&payer_account_info.key.as_ref(), &[bump]]]
//     // )?;

//     // Sign Code Part 
//     //     let pda_account_info = next_account_info(account_info_iter)?;
//     // let to_account_info = next_account_info(account_info_iter)?;
//     // let system_program_account_info = next_account_info(account_info_iter)?;
//     //     let bump_seed = instruction_data[0];
 
//     // invoke_signed(
//     //     &system_instruction::transfer(
//     //         &pda_account_info.key,
//     //         &to_account_info.key,
//     //         100_000_000, // 0.1 SOL
//     //     ),
//     //     &[
//     //         pda_account_info.clone(),
//     //         to_account_info.clone(),
//     //         system_program_account_info.clone(),
//     //     ],
//     //     &[&[b"escrow", &[bump_seed]]],
//     // )?;

//     // close Part
//         let source_account_info = next_account_info(account_info_iter)?;
//     let dest_account_info = next_account_info(account_info_iter)?;
 
//     let dest_starting_lamports = dest_account_info.lamports();
//     **dest_account_info.lamports.borrow_mut() = dest_starting_lamports
//         .checked_add(source_account_info.lamports())
//         .unwrap();
//     **source_account_info.lamports.borrow_mut() = 0;
 
//     let mut source_data = source_account_info.data.borrow_mut();
//     source_data.fill(0);
 
//     Ok(())
// }

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

// 프로그램의 진입점을 정의합니다.
entrypoint!(process_instruction);

// 프로그램의 상태를 저장하는 구조체입니다.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HelloState {
    is_initialized: bool,
}

// 필요한 계정들
/// 1. [서명자] 지불자
/// 2. [쓰기 가능] Hello 상태 계정
/// 3. [] 렌트 계정
/// 4. [] 시스템 프로그램
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // 모든 계정을 반복자로 가져옵니다. 이는 계정을 순차적으로 처리할 수 있게 해줍니다.
    let accounts_iter = &mut accounts.iter();

    // next_account_info를 사용하여 각 계정을 순서대로 가져옵니다.
    // 이 방법은 수동으로 인덱스를 관리하는 것보다 안전하고 편리합니다.

    // 지불자 계정을 가져옵니다.
    let payer_account = next_account_info(accounts_iter)?;
    // Hello 상태 계정을 가져옵니다.
    let hello_state_account = next_account_info(accounts_iter)?;
    // 렌트 계정을 가져옵니다.
    let rent_account = next_account_info(accounts_iter)?;
    // 시스템 프로그램 계정을 가져옵니다.
    let system_program = next_account_info(accounts_iter)?;

    // 여기서는 계정들을 단순히 읽기만 하고 아무 작업도 수행하지 않습니다.
    // 실제 프로그램에서는 이 계정들을 사용하여 다양한 작업을 수행할 수 있습니다.

    Ok(())
}