use {
  crate::{
      instruction::WhitelistInstruction,
      state::WhiteListData,
  },
  borsh::{BorshDeserialize, BorshSerialize},
  solana_program::{
      account_info::{next_account_info, AccountInfo},
      entrypoint::ProgramResult,
      msg,
      program::invoke_signed,
      program::invoke,
      program_error::ProgramError,
      pubkey::Pubkey,
      sysvar::Sysvar,
      sysvar::rent::Rent,
      system_instruction,
  },
  std::convert::TryInto,
};
 
pub fn process_instruction(
  _program_id: &Pubkey,
  accounts: &[AccountInfo],
  input: &[u8],
) -> ProgramResult {
  // Length = BOOL + VEC + Pubkey * n (n = number of keys)
  const INITIAL_ACCOUNT_LEN: usize = 1 + 4 + 0 ;
  msg!("input: {:?}", input);
 
  let instruction = WhitelistInstruction::try_from_slice(input)?;
 
  let accounts_iter = &mut accounts.iter();
 
  let funding_account = next_account_info(accounts_iter)?;
  let pda_account = next_account_info(accounts_iter)?;
  let system_program = next_account_info(accounts_iter)?;
 
  match instruction {
    WhitelistInstruction::Initialize => {
      msg!("Initialize");
 
      let (pda, pda_bump) = Pubkey::find_program_address(
          &[
            b"customaddress",
            &funding_account.key.to_bytes(),
          ],
          _program_id,
      );
 
      let signers_seeds: &[&[u8]; 3] = &[
          b"customaddress",
          &funding_account.key.to_bytes(),
          &[pda_bump],
      ];
 
      if pda.ne(&pda_account.key) {
          return Err(ProgramError::InvalidAccountData);
      }
 
      let lamports_required = Rent::get()?.minimum_balance(INITIAL_ACCOUNT_LEN);
      let create_pda_account_ix = system_instruction::create_account(
          &funding_account.key,
          &pda_account.key,
          lamports_required,
          INITIAL_ACCOUNT_LEN.try_into().unwrap(),
          &_program_id,
      );
 
      invoke_signed(
          &create_pda_account_ix,
          &[
              funding_account.clone(),
              pda_account.clone(),
              system_program.clone(),
          ],
          &[signers_seeds],
      )?;
 
      let mut pda_account_state = WhiteListData::try_from_slice(&pda_account.data.borrow())?;
 
      pda_account_state.is_initialized = true;
      pda_account_state.white_list = Vec::new();
      pda_account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
      Ok(())
    }
    WhitelistInstruction::AddKey { key } => {
      msg!("AddKey");
 
      let mut pda_account_state = WhiteListData::try_from_slice(&pda_account.data.borrow())?;
 
      if !pda_account_state.is_initialized {
          return Err(ProgramError::InvalidAccountData);
      }
 
      let new_size = pda_account.data.borrow().len() + 32;
 
      let rent = Rent::get()?;
      let new_minimum_balance = rent.minimum_balance(new_size);
 
      let lamports_diff = new_minimum_balance.saturating_sub(pda_account.lamports());
      invoke(
          &system_instruction::transfer(funding_account.key, pda_account.key, lamports_diff),
          &[
              funding_account.clone(),
              pda_account.clone(),
              system_program.clone(),
          ],
      )?;
 
      pda_account.realloc(new_size, false)?;
 
      pda_account_state.white_list.push(key);
      pda_account_state.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
 
      Ok(())
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_initialize_and_add_key() {
        // 프로그램 ID 생성
        let program_id = Pubkey::new_unique();

        // Funding 계정 생성
        let funding_key = Pubkey::new_unique();
        let mut funding_lamports = 1000000000; // 충분히 큰 값으로 설정
        let mut funding_data = vec![0; 0];
        let funding_account = AccountInfo::new(
            &funding_key,
            true,
            true,
            &mut funding_lamports,
            &mut funding_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // PDA 계정 생성
        let (pda, _) = Pubkey::find_program_address(&[b"customaddress", &funding_key.to_bytes()], &program_id);
        let mut pda_lamports = 1000000000; // 충분히 큰 값으로 설정
        let mut pda_data = vec![0; 1000];
        let pda_account = AccountInfo::new(
            &pda,
            false,
            true,
            &mut pda_lamports,
            &mut pda_data,
            &program_id,
            false,
            Epoch::default(),
        );

        // System program 계정
        let system_program_key = solana_program::system_program::id();
        let mut system_lamports = 1000000000; // 충분히 큰 값으로 설정
        let mut system_data = vec![];
        let system_program = AccountInfo::new(
            &system_program_key,
            false,
            false,
            &mut system_lamports,
            &mut system_data,
            &system_program_key,
            false,
            Epoch::default(),
        );

        let accounts = vec![funding_account, pda_account, system_program];

        // Initialize 명령 테스트
        let init_instruction = WhitelistInstruction::Initialize;
        let mut init_data = Vec::new();
        init_instruction.serialize(&mut init_data).unwrap();

        let result = process_instruction(&program_id, &accounts, &init_data);
        assert!(result.is_ok(), "Initialize failed: {:?}", result);

        // AddKey 명령 테스트
        let new_key = Pubkey::new_unique();
        let add_key_instruction = WhitelistInstruction::AddKey { key: new_key };
        let mut add_key_data = Vec::new();
        add_key_instruction.serialize(&mut add_key_data).unwrap();

        let result = process_instruction(&program_id, &accounts, &add_key_data);
        assert!(result.is_ok(), "AddKey failed: {:?}", result);

        // 화이트리스트 상태 확인
        let whitelist_data = WhiteListData::try_from_slice(&accounts[1].data.borrow()).unwrap();
        assert!(whitelist_data.is_initialized);
        assert!(whitelist_data.white_list.contains(&new_key));
    }
}