use codec::{Encode, Decode};

use frame_support::debug::{error, native};
use frame_support::traits::Randomness;
use pallet_contracts::chain_extension::{
    ChainExtension, Environment, Ext, InitState, RetVal, SysConfig, UncheckedFrom,
};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

/// contract extension for `FetchRandom`
pub struct KylinExtension;

impl ChainExtension for KylinExtension {
    fn call<E: Ext>(func_id: u32, env: Environment<E, InitState>) -> Result<RetVal, DispatchError>
        where
            <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {

        match func_id {
            1 => {
                let mut env = env.buf_in_buf_out();
                let info = super::KylinOracleModule::requested_offchain_data().0;
                native::trace!(
                    target: "runtime",
                    "[ChainExtension]|call|func_id:{:}",
                    func_id
                );
                env.write(&info.data, false, None)
                    .map_err(|_| DispatchError::Other("ChainExtension failed to call"))?;
            }

            _ => {
                error!("call an unregistered `func_id`, func_id:{:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"));
            }
        }
        Ok(RetVal::Converging(0))
    }

    fn enabled() -> bool {
        true
    }
}
