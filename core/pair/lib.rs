#![cfg_attr(not(feature = "std"), no_std)]

/// Exporting interface for interaction
pub use self::pair::{
	Pair,
	PairRef,
};

use std::num::sqrt;

#[ink::contract]
mod pair {

    /// This would be used to interaction with the PSP22 token in this pool    
    use openbrush::{
        contracts::{
            traits::psp22::PSP22Ref,
        },
    };

    use ink::{storage::Mapping, primitives::AccountId};
    use ink::env::CallFlags;
    use ink::prelude::vec;




    /// ============================
    /// PSP22 pair contract storage
    /// ============================ 
    #[ink(storage)]
    pub struct Pair {
        // ====================================
        // LP TOKEN DATA
        // ====================================
        total_supply: Balance, // this is the total supply of all the LP token that has been minted from this pull
        balances: Mapping<AccountId, Balance>, // this is the mapping of lp token balances 
        lp_tokens_allowances: Mapping<(AccountId,AccountId), Balance>, // this is a 3d mapping of lp token allowances 


        // =====================================
        // PAIR STORAGE DATA
        // =====================================
        factory: AccountId,
        token_one: AccountId, // first token in the pool
        token_two: AccountId, // second token in this pool
        reserve_token_one: Balance,
        reserve_token_two: Balance,
        last_update_time: Balance,
        last_constant_product: Balance
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
    /// EVENTS
    /// ==========================================
    #[ink(event)]
    pub struct Mint {
        #[ink(topic)]
        provider:AccountId,
        token_one_amount:Balance,
        token_two_amount: Balance,
    }


    #[ink(event)]
    pub struct Burn {
        #[ink(topic)]
        provider:AccountId,
        token_one_amount:Balance,
        token_two_amount: Balance,
        receiver: AccountId
    }


    #[ink(event)]
    pub struct Swap {
        #[ink(topic)]
        trader: AccountId,
        amount_token_one_in:Balance,
        amount_token_two_in: Balance,
        amount_token_one_out:Balance,
        amount_token_two_out: Balance,
        receiver: AccountId
    }


    #[ink(event)]
    pub struct Sync {
        token_one_reserve: Balance,
        token_two_reserve: Balance
    }


    #[ink(impl)]
    impl Pair {
        // ========================
        // INTERNAL FUNCTIONS
        // ========================

        fn _update(&mut self, _reserve_token_one: Balance, _reserve_token_two: Balance) -> Result<(), PairErrors> {
            self.reserve_token_one = _reserve_token_one;
            self.reserve_token_two = _reserve_token_two;
            self.last_update_time = self.env().block_number();

            self.env().emit_event(
                    Sync {
                        token_one_reserve: _reserve_token_one,
                        token_two_reserve: _reserve_token_two
                    }
                );
            Ok(())
        }

        fn _balance_of(&self, token: AccountId, account: AccountId) -> Balance {
            PSP22Ref::balance_of(
                &token,
                account
            )
        }

        fn burn_address(&self) -> AccountId {
            [1u8; 32].into()
        }


        // ADD a mint fee function here is you want fee to be paid to the protocol managers (Look into Uniswap v2 business model for more details)
    }


    /// ==========================================
    /// PSP22 pair contract logic implementation
    /// ==========================================
    impl Pair {
        const MINIMUM_LIQUIDITY: Balance = 1000;

        #[ink(constructor)]
        pub fn new(
            token_one:AccountId,
            token_two:AccountId,
            fee: Balance,
            fee_vault:AccountId
        ) -> Self {
            let balances = Mapping::default();
            let lp_tokens_allowances = Mapping::default();
            let total_supply = 0;
            let traders_fee:Balance = 25;

            Self {
                token_one,
                token_two,
                fee,
                total_supply,
                balances,
                lp_tokens_allowances,
                fee_vault,
                reserve_token_one,
                reserve_token_two,
                last_update_time
            }
        }

        
        /// This function would be used to add liquidity this pool
        #[ink(message)]
        pub fn mint(&mut self, to: AccountId)  -> Result<(balance), PairErrors> {
            let caller = self.env().caller();
            let (reserve_token_one, reserve_token_two, last_update_time) = self.get_reserves();
            let address_this = self.env().account_id();
            
            let pair_token_one_balance = _balance_of(self.token_one, address_this);
            let pair_token_two_balance = _balance_of(self.token_two, address_this);

            let amount_in_token_one = pair_token_one_balance - reserve_token_one;
            let amount_in_token_two = pair_token_two_balance - reserve_token_two;
            let liquidity: Balance;


            if self.total_supply == 0 {
                // in this case liquidity is benin provided for the first 
                liquidity = sqrt(amount_token_one * amount_token_two) - MINIMUM_LIQUIDITY;
                
            }
            
            Ok(())
        }


        #[ink(message)]
        pub fn burn(&mut self, to: AccountId) -> Result<(Balance, Balance), PairErrors> {
            let amount_token_one = (balance_lp_in_pool * pair_token_one_balance) / self.total_supply;
            let amount_token_two = (balance_lp_in_pool * pair_token_two_balance) / self.total_supply;
        }

        
        #[ink(message)]
        pub fn get_reserves(&self) -> Result<(Balance, Balance, Balance), PairErrors> {
            Ok((
                self.reserve_token_one,
                self.reserve_token_two,
                self.last_update_time
            ))
        }
    }
}