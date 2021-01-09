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
        // Store dataId's service name
        service_name_map: StorageHashMap<u64, Vec<u8>>,
        // Store dataId's service owner
        service_owner_map: StorageHashMap<u64, AccountId>,
        // Store dataId's service desc
        service_desc_map: StorageHashMap<u64, Vec<u8>>,
        // Store dataId's service thumb
        service_thumb_map: StorageHashMap<u64, Vec<u8>>,
    }

    // struct ServiceInfo {
    //     data_id: u64,
    //     name: Vec<u8>,
    //     desc: Vec<u8>,
    //     thumb: Vec<u8>,
    // }

    impl OracleMarket {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                service_owner_map: Default::default(),
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
            self.service_owner_map.insert(data_id, account_id);
            self.service_name_map.insert(data_id, name);
            self.service_desc_map.insert(data_id, desc);
            self.service_thumb_map.insert(data_id, thumb);
        }

        /// query service's owner
        #[ink(message)]
        pub fn query_service_owner(&self, data_id: u64) -> AccountId {
            let account_id = Self::env().caller();
            let account_id = self.service_owner_map.get(&data_id).copied().unwrap_or(account_id);
            account_id
        }

        /// query service's info
        // #[ink(message)]
        // pub fn query_service_by_data_id(&self, data_id: u64) -> ServiceInfo {
        //     let data_id = self.service_data_id_map.get(&of).copied().unwrap_or(0);
        //     let name = self.service_name_map.get(&of).unwrap();
        //     let desc = self.service_desc_map.get(&of).unwrap();
        //     let thumb = self.service_thumb_map.get(&of).unwrap();
        //     return ServiceInfo{
        //         data_id,
        //         name: name.to_vec(),
        //         desc: desc.to_vec(),
        //         thumb: thumb.to_vec(),
        //     }
        // }

        /// query service's name
        #[ink(message)]
        pub fn query_service_name(&self, data_id: u64) -> Vec<u8> {
            let name = self.service_name_map.get(&data_id).unwrap();
            name.to_vec()
        }

        /// query service's desc
        #[ink(message)]
        pub fn query_service_desc(&self, data_id: u64) -> Vec<u8> {
            let desc = self.service_desc_map.get(&data_id).unwrap();
            desc.to_vec()
        }

        /// query service's thumb
        #[ink(message)]
        pub fn query_service_thumb(&self, data_id: u64) -> Vec<u8> {
            let thumb = self.service_thumb_map.get(&data_id).unwrap();
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
