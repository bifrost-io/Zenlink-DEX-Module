use super::{Config, Weight, KLast};
use frame_support::traits::Get;
use sp_core::U256;

pub fn update_k_value_type_to_u256<T: Config>() -> Weight {
    KLast::<T>::translate_values::<u128, _>(|v| Some(U256::from(v)));

    T::DbWeight::get().writes(4) + T::DbWeight::get().writes(4)
}