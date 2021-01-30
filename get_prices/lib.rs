#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::{vec::Vec};

/// Custom chain extension to read to and write from the runtime.
#[ink::chain_extension]
pub trait KylinOracleModule {
    type ErrorCode = ModuleError;

    /// Reads from runtime storage.
    #[ink(extension = 1, returns_result = false)]
    fn requested_offchain_data(data_id: u64) -> Vec<u8>;
}

#[derive(
Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ModuleError {
    QueryError,
}

impl ink_env::chain_extension::FromStatusCode for ModuleError {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::QueryError),
            _ => panic!("encountered unknown status code"),
        }
    }
}

pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type ChainExtension = KylinOracleModule;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod get_prices {
    use super::{ModuleError};

    use ink_prelude::{vec::Vec};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct GetPrices {}

    impl GetPrices {

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn requested_offchain_data(&self, data_id: u64) -> Vec<u8> {
            let res = self.env()
                .extension()
                .requested_offchain_data(data_id);
            res.unwrap()
        }
    }
}
