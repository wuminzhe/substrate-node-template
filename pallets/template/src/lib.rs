#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, ensure};
use frame_support::traits::{Randomness};
use system::ensure_signed;
use sp_std::prelude::Vec;
use codec::{Codec, Decode, Encode};
use sp_core::H256;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Entity<Hash> {
    id: Hash,
    status: u8,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.
	type Randomness: Randomness<Self::Hash>;

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// entities
		Entities get(entities): map hasher(blake2_128_concat) T::Hash => Entity<T::Hash>;
		EntitiesArray get(entity_by_index): map hasher(twox_64_concat) u64 => T::Hash;
		EntitiesCount get(entities_count): u64;
		EntitiesIndex: map hasher(blake2_128_concat) T::Hash => u64;

		// managers and issuers of entity
		EntityManagers get(fn entity_managers): map hasher(blake2_128_concat) T::Hash => Vec<T::AccountId>;
		EntityIssuers get(fn entity_issuers): map hasher(blake2_128_concat) T::Hash => Vec<T::AccountId>;

		// certificates
		CertificatesArray get(certificate_by_index): map hasher(twox_64_concat) u64 => Vec<u8>;
		CertificatesCount get(certificates_count): u64;
		CertificatesIndex: map hasher(blake2_128_concat) Vec<u8> => u64;

		// certificate's entity id
		EntityOfCertificate get(fn entity_of_certificate): map hasher(blake2_128_concat) Vec<u8> => T::Hash;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		pub fn create_entity(origin) -> dispatch::DispatchResult {
			let creator = ensure_signed(origin)?;
			let new_hash = T::Randomness::random_seed();
			ensure!(!<Entities<T>>::contains_key(new_hash), "Entity already exists");

			let entities_count = Self::entities_count();
			let new_entities_count = entities_count.checked_add(1).ok_or("Overflow adding a new entity to total supply")?;

			let new_entity = Entity {
				id: new_hash,
				status: 1,
			};
			<Entities<T>>::insert(new_hash, new_entity);
			<EntitiesArray<T>>::insert(entities_count, new_hash);
			<EntitiesCount>::put(new_entities_count);
			<EntitiesIndex<T>>::insert(new_hash, entities_count);

			// creator will be manager
			let mut entity_managers = Self::entity_managers(new_hash);
			entity_managers.push(creator.clone());
			<EntityManagers<T>>::insert(new_hash, entity_managers);

			// creator will be issuer
			let mut entity_issuers = Self::entity_issuers(new_hash);
			entity_issuers.push(creator);
			<EntityIssuers<T>>::insert(new_hash, entity_issuers);

			Ok(())
		}

		pub fn create_certificate(origin, issuer_id: T::AccountId, nonce: u64, certificate: Vec<u8>) {
			let sender = ensure_signed(origin)?;
			// TODO: sender must be a publisher
			
			let version = &certificate[0];
			let entity_id: T::Hash = H256::from_slice(&certificate[1..34]);
			let hash = H256::from_slice(&certificate[34..67]);
			// let sigature = 

			let certificates_count = Self::certificates_count();
			let new_certificates_count = certificates_count.checked_add(1).ok_or("Overflow adding a new certificate to total supply")?;

			<CertificatesArray>::insert(certificates_count, certificate.clone());
			<CertificatesCount>::put(new_certificates_count);
			<CertificatesIndex>::insert(certificate, certificates_count);


			<EntityOfCertificate<T>>::insert(certificate, entity_id);
		}
	}
}
