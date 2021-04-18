#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{ensure, decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Proofs get(fn proofs): map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
		ClaimRemoved(AccountId, Vec<u8>),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		ProofAlreadyExist,
		ClaimNotExist,
		NotClaimOwner,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			// 确保交易被某用户签名，并获取签名ID
			let sender = ensure_signed(origin)?;
			// 验证
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);
			// 存储单元保存
			Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Module::<T>::block_number()));
			// 触发事件
			Self::deposit_event(RawEvent::ClaimCreated(sender, claim));
			// 交易成功
			Ok(())
		}

		#[weight = 0]
		pub fn revoke_claim(origin, claim: Vec<u8>) -> dispatch::DispatchResult {
			// 确保交易被某用户签名，并获取签名ID
			let sender = ensure_signed(origin)?;
			// 确保存证存在
			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			// _block_number区块数，未用到，用_避免编译出错
			let (owner, _block_number) = Proofs::<T>::get(&claim);
			// 确保用户
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			// 存储单元撤销
			Proofs::<T>::remove(&claim);
			// 触发事件
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
			// 撤销交易成功
			Ok(())
		}

		#[weight = 0]
		pub fn move_claim(origin, claim: Vec<u8>, account: T::AccountId) -> dispatch::DispatchResult {
			// 确保交易被某用户签名，并获取签名ID
			let sender = ensure_signed(origin)?;
			// 确保存证存在
			ensure!(Proofs::<T>::contains_key(&claim), Error::<T>::ClaimNotExist);
			// _block_number区块数，未用到，用_避免编译出错
			let (owner, _block_number) = Proofs::<T>::get(&claim);
			// 确保用户
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			// 存储单元转移
			Proofs::<T>::insert(&claim, (account.clone(), frame_system::Module::<T>::block_number()));
			// 触发事件
			Self::deposit_event(RawEvent::ClaimRemoved(account, claim));
			// 交易成功
			Ok(())
		}
	}
}
