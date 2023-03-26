#![cfg_attr(not(feature = "std"), no_std)]

/// Exporting interface for interaction
pub use self::pair::{
	TradingPairPsp22,
	TradingPairPsp22Ref,
};



#[ink::contract]
mod pair {

    /// This would be used to interaction with the PSP22 token in this pool    
    use openbrush::{
        contracts::{

            traits::psp22::PSP22Ref,
        },
    };

    use ink::storage::Mapping;
    use ink::env::CallFlags;
    use ink::prelude::vec;




    /// ============================
    /// PSP22 pair contract storage
    /// ============================ 
    #[ink(storage)]
    pub struct Pair {
        token_one: AccountId, // first token in the pool
        token_two: AccountId, // second token in this pool
        fee: Balance, // this is the fee that would be paid to this protocol
        total_supply: Balance, // this is the total supply of all the LP token that has been minted from this pull
        balances: Mapping<AccountId, Balance>, // this is the mapping of lp token balances 
        lp_tokens_allowances: Mapping<(AccountId,AccountId), Balance>, // this is a 3d mapping of lp token allowances 
        fee_vault: AccountId // this is the address that would be receiving the fee from swaps 
    }



    /// ==========================================
    /// ERRORS
    /// ==========================================
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum PairErrors {
        InsufficientTokenOneBalance,
        InsufficientTokenTwoBalance,
        InsufficientTokenOneAllowance,
        InsufficientTokenTwoAllowance,
        Overflow,
        ZeroLPMinted,
        SlippageTolerance,
        TokenOneTransferFromFailed,
        TokenTwoTransferFromFailed,
        TokenOneTransferFailed,
        TokenTwoTransferFailed,
        InsufficientLPBalance,
        PoolOutOfTokenOne,
        PoolOutOfTokenTwo,
        InsufficientLPAllowance
    }

    

    /// ==========================================
    /// PSP22 pair contract logic implementation
    /// ==========================================
    impl Pair {
        
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}