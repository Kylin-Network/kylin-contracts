#![cfg_attr(not(feature = "std"), no_std)]

use ink_env::Environment;
use ink_lang as ink;
use ink_prelude::{vec::Vec};

/// Custom chain extension to read to and write from the runtime.
#[ink::chain_extension]
pub trait KylinOracleModule {
    type ErrorCode = ModuleErrorCode;

    /// Reads from runtime storage.
    #[ink(extension = 1)]
    fn dataId() -> Result<u64, ModuleError>;

    /// Reads from runtime storage.
    #[ink(extension = 2)]
    fn requestedOffchainData(dataId: u64) -> Result<Vec<u8>, ModuleError>;
}

/// The shared error code for the read write chain extension.
#[derive(
Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo,
)]
pub enum ModuleErrorCode {
    InvalidKey,
    CannotWriteToKey,
    CannotReadFromKey,
}


/// Returned by `read_small` in case there were too few bytes available in the buffer.
///
/// Provides the number of bytes required to read the storage cell.
#[derive(
Debug, Copy, Clone, PartialEq, Eq, scale::Encode, scale::Decode, scale_info::TypeInfo,
)]
pub enum ModuleError {
    ErrorCode(ModuleErrorCode),
    BufferTooSmall { required_bytes: u32 },
}

impl From<ModuleErrorCode> for ModuleError {
    fn from(error_code: ModuleErrorCode) -> Self {
        Self::ErrorCode(error_code)
    }
}

impl From<scale::Error> for ModuleError {
    fn from(_: scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink_env::chain_extension::FromStatusCode for ModuleErrorCode {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::InvalidKey),
            2 => Err(Self::CannotWriteToKey),
            3 => Err(Self::CannotReadFromKey),
            _ => panic!("encountered unknown status code"),
        }
    }
}

pub enum CustomEnvironment {}

impl Environment for CustomEnvironment {
    const MAX_EVENT_TOPICS: usize =
        <ink_env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink_env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink_env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink_env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink_env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink_env::DefaultEnvironment as Environment>::Timestamp;
    type ChainExtension = KylinOracleModule;
}

#[ink::contract(env = crate::CustomEnvironment)]
mod get_prices {
    use super::{ModuleErrorCode, ModuleError};

    use ink_prelude::{vec::Vec};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct GetPrices {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    /// Copy of the custom type defined in `/runtime/src/template.rs`.
    ///
    /// # Requirements
    /// In order to decode a value of that type from the runtime storage:
    ///   - The type must match exactly the custom type defined in the runtime
    ///   - It must implement `Decode`, usually by deriving it as below
    ///   - It should implement `Metadata` for use with `generate-metadata` (required for the UI).
    //    #[derive(scale::Decode, scale::Encode)]
    //    #[cfg_attr(feature = "ink-generate-abi", derive(type_metadata::Metadata))]
    //    pub struct Foo {
    //        id: u32,
    //        data: Vec<u8>,
    //    }

    impl GetPrices {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn dataId(&self) -> Result<u64, ModuleError> {
            self.env()
                .extension()
                .dataId()
        }

        #[ink(message)]
        pub fn requestedOffchainData(&self, dataId: u64) -> Result<Vec<u8>, ModuleError> {
            self.env()
                .extension()
                .requestedOffchainData(dataId)
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }
}
