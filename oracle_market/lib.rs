#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod oracle_market {

    use ink_prelude::vec::Vec;
    use ink_storage::collections::{HashMap as StorageHashMap, Vec as StorageVec};
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct OracleMarket {
        // Store a contract owner
        owner: AccountId,
        // Store accountId's service name
        service_name_map: StorageHashMap<AccountId, Vec<u8>>,
        // Store accountId's service dataId
        service_data_id_map: StorageHashMap<AccountId, u64>,
        // Store accountId's service desc
        service_desc_map: StorageHashMap<AccountId, Vec<u8>>,
        // Store accountId's service desc
        service_thumb_map: StorageHashMap<AccountId, Vec<u8>>,
    }

    impl OracleMarket {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                service_data_id_map: Default::default(),
                service_name_map: Default::default(),
                service_desc_map: Default::default(),
                service_thumb_map: Default::default(),
            }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn add_service(&mut self, data_id: u64, name: Vec<u8>, desc: Vec<u8>, thumb: Vec<u8>) {
            let account_id = Self::env().caller();
            self.service_data_id_map.insert(account_id, data_id);
            self.service_name_map.insert(account_id, name);
            self.service_desc_map.insert(account_id, desc);
            self.service_thumb_map.insert(account_id, thumb);
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn query_service_data_id(&self, of: AccountId) -> u64 {
            let data_id = self.service_data_id_map.get(&of).copied().unwrap_or(0);
            data_id
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn query_service_name(&self, of: AccountId) -> Vec<u8> {
            let name = self.service_name_map.get(&of).unwrap();
            name.to_vec()
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn query_service_desc(&self, of: AccountId) -> Vec<u8> {
            let desc = self.service_desc_map.get(&of).unwrap();
            desc.to_vec()
        }

        /// A message that init a service.
        #[ink(message)]
        pub fn query_service_thumb(&self, of: AccountId) -> Vec<u8> {
            let thumb = self.service_thumb_map.get(&of).unwrap();
            thumb.to_vec()
        }

        //        /// A message that can be called on instantiated contracts.
        //        /// This one flips the value of the stored `bool` from `true`
        //        /// to `false` and vice versa.
        //        #[ink(message)]
        //        pub fn flip(&mut self) {
        //            self.value = !self.value;
        //        }
        //
        //        /// Simply returns the current value of our `bool`.
        //        #[ink(message)]
        //        pub fn get(&self) -> bool {
        //            self.value
        //        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn default_works() {
            let oracle_market = OracleMarket::default();
            // assert_eq!(oracle_market.get(), false);
        }
        //
        //        /// We test a simple use case of our contract.
        //        #[test]
        //        fn it_works() {
        //            let mut oracle_market = OracleMarket::new(false);
        //            assert_eq!(oracle_market.get(), false);
        //            oracle_market.flip();
        //            assert_eq!(oracle_market.get(), true);
        //        }
    }
}
