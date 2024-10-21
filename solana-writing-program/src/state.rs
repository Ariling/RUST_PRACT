use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WhiteListData {
    pub is_initialized: bool,
    pub white_list: Vec<Pubkey>,
}