#![allow(unused_imports)]
#![allow(unused_variables)]

use codec::{Decode, Encode};
use rstd::{convert::TryInto, prelude::*, result};
use sr_primitives::traits::{CheckedDiv, CheckedSub, Hash};
use support::{
    decl_event, decl_module, decl_storage, dispatch::Result, ensure, traits::Currency, StorageMap, StorageValue,
};

use log;
use rstd::convert::Into;
use system::ensure_signed;


pub trait Trait: system::Trait {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}


#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default)]
pub struct ZXInfo<TAccountId> {
    pub owner: TAccountId,
    pub data1: u64,
    pub data2: u64,
    pub data3: u64,
    pub data4: u64
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as system::Trait>::AccountId,
    {
        /// Deposit a new event
        ItemAdded(AccountId, u64, u64, u64, u64),
    }
);

decl_storage! {
    trait Store for Module<T: Trait> as ZXModule {
        Items: map T::AccountId => ZXInfo<T::AccountId>;
    }
}

decl_module! {
    /// The module declaration.
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initializing events
        // this is needed only if you are using events in your module
        fn deposit_event() = default;

        /// Create a new zx
        pub fn new(origin, data1: u64, data2: u64, data3: u64, data4: u64) -> Result {
            let owner = ensure_signed(origin)?;

            let one_zx = ZXInfo {
                owner: owner.clone(),
                data1,
                data2,
                data3,
                data4
            };

            <Items<T>>::insert(owner.clone(), one_zx);

            Self::deposit_event(RawEvent::ItemAdded(owner.clone(), data1, data2, data3, data4));
            Ok(())
        }

    }
}
