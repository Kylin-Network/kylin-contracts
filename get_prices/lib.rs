#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

/// Define hashing functions required for hashing the key to read a Value from runtime storage
mod hashing {
    /// Do a XX 128-bit hash and place result in `dest`.
    pub fn twox_128_into(data: &[u8], dest: &mut [u8; 16]) {
        use ::core::hash::Hasher;
        let mut h0 = twox_hash::XxHash::with_seed(0);
        let mut h1 = twox_hash::XxHash::with_seed(1);
        h0.write(data);
        h1.write(data);
        let r0 = h0.finish();
        let r1 = h1.finish();
        use byteorder::{ByteOrder, LittleEndian};
        LittleEndian::write_u64(&mut dest[0..8], r0);
        LittleEndian::write_u64(&mut dest[8..16], r1);
    }

    /// Do a XX 128-bit hash and return result.
    pub fn twox_128(data: &[u8]) -> [u8; 16] {
        let mut r: [u8; 16] = [0; 16];
        twox_128_into(data, &mut r);
        r
    }
}

#[ink::contract]
mod get_prices {

    use super::hashing;
    use ink_prelude::{format, vec::Vec};

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

        /// Attempts to read an instance of the custom struct from runtime storage
        ///
        /// Returns `None` if the key does not exist, or it failed to decode the value.
        #[ink(message)]
        pub fn read_custom_runtime(&self) -> Option<Vec<u32>> {
            let mut key = [0u8; 32];
            // A storage key is constructed as `Twox128(module_prefix) ++ Twox128(storage_prefix)`
            let module_prefix = hashing::twox_128(&b"PriceFetchModule"[..]);
            let storage_prefix = hashing::twox_128(&b"Prices"[..]);
            key[0..16].copy_from_slice(&module_prefix);
            key[16..32].copy_from_slice(&storage_prefix);
            //            env::println(&format!("Storage key: {:?}", key));

            // Attempt to read and decode the value directly from the runtime storage
            let result = self.env().get_runtime_storage::<Vec<u32>>(&key[..]);
            match result {
                Some(foo) => {
                    match foo {
                        Ok(foo) => {
                            // Return the successfully decoded instance of `Foo`
                            Some(foo)
                        }
                        Err(err) => {
                            // Error decoding the value at Foo.
                            //                            env::println(&format!("Error reading runtime storage: {:?}", err));
                            None
                        }
                    }
                }
                None => {
                    // Key not present
                    //                    env::println(&format!("No such key: {:?}", key));
                    None
                }
            }
        }
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
            let get_prices = GetPrices::default();
            assert_eq!(get_prices.get(), false);
        }

        /// We test a simple use case of our contract.
        #[test]
        fn it_works() {
            let mut get_prices = GetPrices::new(false);
            assert_eq!(get_prices.get(), false);
            get_prices.flip();
            assert_eq!(get_prices.get(), true);
        }
    }
}
