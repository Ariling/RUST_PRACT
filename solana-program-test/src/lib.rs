use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint,
    entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey, system_instruction, sysvar::{rent::Rent, Sysvar}
};
 
entrypoint!(process_instruction);
 
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
 
    // let payer_account_info = next_account_info(account_info_iter)?;
    // let pda_account_info = next_account_info(account_info_iter)?;
    // let rent_sysvar_account_info = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    // creation code Part
 
    // find space and minimum rent required for account
    // let space = instruction_data[0];
    // let bump = instruction_data[1];
    // let rent_lamports = rent_sysvar_account_info.minimum_balance(space.into());
 
    // invoke_signed(
    //     &system_instruction::create_account(
    //         &payer_account_info.key,
    //         &pda_account_info.key,
    //         rent_lamports,
    //         space.into(),
    //         program_id
    //     ),
    //     &[
    //         payer_account_info.clone(),
    //         pda_account_info.clone()
    //     ],
    //     &[&[&payer_account_info.key.as_ref(), &[bump]]]
    // )?;

    // Sign Code Part 
    //     let pda_account_info = next_account_info(account_info_iter)?;
    // let to_account_info = next_account_info(account_info_iter)?;
    // let system_program_account_info = next_account_info(account_info_iter)?;
    //     let bump_seed = instruction_data[0];
 
    // invoke_signed(
    //     &system_instruction::transfer(
    //         &pda_account_info.key,
    //         &to_account_info.key,
    //         100_000_000, // 0.1 SOL
    //     ),
    //     &[
    //         pda_account_info.clone(),
    //         to_account_info.clone(),
    //         system_program_account_info.clone(),
    //     ],
    //     &[&[b"escrow", &[bump_seed]]],
    // )?;

    // close Part
        let source_account_info = next_account_info(account_info_iter)?;
    let dest_account_info = next_account_info(account_info_iter)?;
 
    let dest_starting_lamports = dest_account_info.lamports();
    **dest_account_info.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(source_account_info.lamports())
        .unwrap();
    **source_account_info.lamports.borrow_mut() = 0;
 
    let mut source_data = source_account_info.data.borrow_mut();
    source_data.fill(0);
 
    Ok(())
}