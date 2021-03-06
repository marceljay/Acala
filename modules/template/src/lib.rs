#![cfg_attr(not(feature = "std"), no_std)]

use support::{decl_event, decl_module, decl_storage, traits::Currency};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Currency: Currency<Self::AccountId>;
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

decl_storage! {
	trait Store for Module<T: Trait> as Template {

	}
}

decl_event!(
	pub enum Event<T> where
		<T as system::Trait>::AccountId,
		Balance = BalanceOf<T>,
	{
		Dummy(AccountId, Balance),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

	}
}

impl<T: Trait> Module<T> {}
