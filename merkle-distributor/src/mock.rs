// Copyright 2021-2022 Zenlink.
// Licensed under Apache 2.0.

//! Mocks for the merkle-distributor
use super::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::BuildStorage;

use crate::mock::TokenSymbol::*;
use frame_support::{construct_runtime, derive_impl, parameter_types, traits::Contains};
use orml_traits::parameter_type_with_key;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	AccountId32, MultiSignature,
};

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	MaxEncodedLen,
	PartialOrd,
	Ord,
	TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
	Token(TokenSymbol),
	Other(TokenSymbol),
}

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	MaxEncodedLen,
	PartialOrd,
	Ord,
	TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenSymbol {
	Test1 = 0,
	Test2 = 1,
}

mod merkle_distributor {
	pub use super::super::*;
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MdPalletId: PalletId = PalletId(*b"zlk/md**");
	pub const StringLimit: u32 = 50;
	pub const MaxReserves: u32 = 50;
	pub const ExistentialDeposit: u64 = 1;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		Default::default()
	};
}

pub(crate) const CURRENCY_TEST1: CurrencyId = CurrencyId::Token(TokenSymbol::Test1);

type Balance = u128;
type Block = frame_system::mocking::MockBlock<Runtime>;

pub type Signature = MultiSignature;
pub type AccountPublic = <Signature as Verify>::Signer;
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

pub const ALICE: AccountId = AccountId32::new([0u8; 32]);
pub const BOB: AccountId = AccountId32::new([1u8; 32]);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<u128>;
}

pub struct DustRemovalWhitelist;

impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(_: &AccountId) -> bool {
		true
	}
}

impl orml_tokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type Amount = i128;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type MaxLocks = ();
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type CurrencyHooks = ();
}

impl pallet_balances::Config for Runtime {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Runtime>;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
}

impl Config for Runtime {
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type MerkleDistributorId = u32;
	type PalletId = MdPalletId;
	type StringLimit = StringLimit;
	type MultiCurrency = Tokens;
	type WeightInfo = ();
}

construct_runtime!(
	pub enum Runtime {
		System: frame_system,
		MerkleDistributor: merkle_distributor,
		Balances: pallet_balances,
		Tokens: orml_tokens,
	}
);

pub type MdPallet = Pallet<Runtime>;

pub(crate) const UNIT: Balance = 1_000_000_000_000;

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Runtime>::default()
		.build_storage()
		.unwrap()
		.into();
	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, 1_000_000 * UNIT), (BOB, 1_000_000 * UNIT)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	orml_tokens::GenesisConfig::<Runtime> {
		balances: vec![
			(ALICE, CURRENCY_TEST1, 1_000_000_000_000 * UNIT),
			(BOB, CurrencyId::Token(Test2), 1_000_000 * UNIT),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}
