// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! XCM configurations for the Millau runtime.

use super::{AccountId, Balances, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, XcmPallet};

use bp_westend::Westend;
use bp_xcm_bridge_hub::{Bridge, BridgeId};
use frame_support::{
    parameter_types,
    traits::{ConstU32, Everything, Nothing},
    weights::Weight,
};
use frame_system::EnsureRoot;
use parachains_common::polkadot::fee::WeightToFee;
// TODO: Check how to gather all and if we keep them
use bridge_hub_westend_runtime::{
    xcm_config::{SafeCallFilter, WestendLocation},
    PolkadotXcm,
};
use parachains_common::impls::ToStakingPot;
use westend_runtime::{xcm_config::ThisNetwork, AllPalletsWithSystem};
use xcm::latest::prelude::*;
use xcm::v3::{NetworkId::Rococo as RococoId, NetworkId::Westend as WestednId};
use xcm_builder::{
    Account32Hash, AccountId32Aliases, CurrencyAdapter as XcmCurrencyAdapter, IsConcrete,
    MintLocation, SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation,
    TakeWeightCredit, UsingComponents, WeightInfoBounds,
};
use xcm_executor::traits::{ExportXcm, WithOriginFilter};

parameter_types! {
    /// The location of the `MLAU` token, from the context of this chain. Since this token is native to this
    /// chain, we make it synonymous with it and thus it is the `Here` location, which means "equivalent to
    /// the context".
    pub const TokenLocation: MultiLocation = Here.into_location();
    /// Token asset identifier.
    pub TokenAssetId: AssetId = TokenLocation::get().into();

    /// Our XCM location ancestry - i.e. our location within the Consensus Universe.
    ///
    /// Since Kusama is a top-level relay-chain with its own consensus, it's just our network ID.
    pub UniversalLocation: InteriorMultiLocation = ThisNetwork::get().into();
    /// The check account, which holds any native assets that have been teleported out and not back in (yet).
    pub CheckAccount: (AccountId, MintLocation) = (XcmPallet::check_account(), MintLocation::Local);
}

/// The canonical means of converting a `MultiLocation` into an `AccountId`, used when we want to
/// determine the sovereign account controlled by a location.
pub type SovereignAccountOf = (
    // We can directly alias an `AccountId32` into a local account.
    AccountId32Aliases<ThisNetwork, AccountId>,
    // Dummy stuff for our tests.
    Account32Hash<ThisNetwork, AccountId>,
);

/// Our asset transactor. This is what allows us to interest with the runtime facilities from the
/// point of view of XCM-only concepts like `MultiLocation` and `MultiAsset`.
///
/// Ours is only aware of the Balances pallet, which is mapped to `TokenLocation`.
pub type LocalAssetTransactor = XcmCurrencyAdapter<
    // Use this currency:
    Balances,
    // Use this currency when it is a fungible asset matching the given location or name:
    IsConcrete<TokenLocation>,
    // We can convert the MultiLocations with our converter above:
    SovereignAccountOf,
    // Our chain's account ID type (we can't get away without mentioning it explicitly):
    AccountId,
    // We track our teleports in/out to keep total issuance correct.
    CheckAccount,
>;

/// The means that we convert the XCM message origin location into a local dispatch origin.
type LocalOriginConverter = (
    // A `Signed` origin of the sovereign account that the original location controls.
    SovereignSignedViaLocation<SovereignAccountOf, RuntimeOrigin>,
    // The AccountId32 location type can be expressed natively as a `Signed` origin.
    SignedAccountId32AsNative<ThisNetwork, RuntimeOrigin>,
);

parameter_types! {
    /// The amount of weight an XCM operation takes. This is a safe overestimate.
    pub const BaseXcmWeight: Weight = Weight::from_parts(1_000_000_000, 64 * 1024);
    /// Maximum number of instructions in a single XCM fragment. A sanity check against weight
    /// calculations getting too crazy.
    pub const MaxInstructions: u32 = 100;
}

/// The XCM router. We are not sending messages to sibling/parent/child chains here.
pub type XcmRouter = ();

/// The barriers one of which must be passed for an XCM message to be executed.
pub type Barrier = (
    // Weight that is paid for may be consumed.
    TakeWeightCredit,
);

/// Dispatches received XCM messages from other chain.
pub type OnMillauBlobDispatcher = xcm_builder::BridgeBlobDispatcher<
    crate::xcm_config::XcmRouter,
    crate::xcm_config::UniversalLocation,
    (),
>;

/// XCM weigher type.
pub type XcmWeigher = xcm_builder::FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
    type RuntimeCall = RuntimeCall;
    type XcmSender = XcmRouter;
    type AssetTransactor = LocalAssetTransactor;
    type OriginConverter = LocalOriginConverter;
    type IsReserve = ();
    type IsTeleporter = ();
    type UniversalLocation = UniversalLocation;
    type Barrier = Barrier;
	type Weigher = xcm_builder::weight::FixedWeightBounds<
        collectives_westend_runtime::xcm_config::TempFixedXcmWeight,
		RuntimeCall,
		MaxInstructions,
	>;
    type Trader =
        UsingComponents<WeightToFee, WestendLocation, AccountId, Balances, ToStakingPot<Runtime>>;
    type ResponseHandler = PolkadotXcm;
    type AssetTrap = PolkadotXcm;
    type AssetLocker = ();
    type AssetExchanger = ();
    type AssetClaims = PolkadotXcm;
    type SubscriptionService = PolkadotXcm;
    type PalletInstancesInfo = AllPalletsWithSystem;
    type MaxAssetsIntoHolding = ConstU32<64>;
    type FeeManager = ();
    type MessageExporter =
        bridge_hub_westend_runtime::bridge_to_rococo_config::ToBridgeHubRococoHaulBlobExporter;
    type UniversalAliases = Nothing;
    type CallDispatcher = WithOriginFilter<SafeCallFilter>;
    type SafeCallFilter = SafeCallFilter;
    type Aliasers = Nothing;
}

/// Type to convert an `Origin` type value into a `MultiLocation` value which represents an interior
/// location of this chain.
pub type LocalOriginToLocation = (
    // Usual Signed origin to be used in XCM as a corresponding AccountId32
    SignedToAccountId32<RuntimeOrigin, AccountId, ThisNetwork>,
);

#[cfg(feature = "runtime-benchmarks")]
parameter_types! {
    pub ReachableDest: Option<MultiLocation> = None;
}

impl pallet_xcm::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    // We don't allow any messages to be sent via the transaction yet. This is basically safe to
    // enable, (safe the possibility of someone spamming the parachain if they're willing to pay
    // the DOT to send from the Relay-chain). But it's useless until we bring in XCM v3 which will
    // make `DescendOrigin` a bit more useful.
    type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmRouter = ();
    // Anyone can execute XCM messages locally.
    type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
    type XcmExecuteFilter = Everything;
    type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
    // Anyone is able to use teleportation regardless of who they are and what they want to
    // teleport.
    type XcmTeleportFilter = Everything;
    // Anyone is able to use reserve transfers regardless of who they are and what they want to
    // transfer.
    type XcmReserveTransferFilter = Everything;
    type Weigher = XcmWeigher;
    type UniversalLocation = UniversalLocation;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
    type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
    type Currency = Balances;
    type CurrencyMatcher = ();
    type TrustedLockers = ();
    type SovereignAccountOf = SovereignAccountOf;
    type MaxLockers = frame_support::traits::ConstU32<8>;
    type WeightInfo = pallet_xcm::TestWeightInfo;
    #[cfg(feature = "runtime-benchmarks")]
    type ReachableDest = ReachableDest;
    type AdminOrigin = EnsureRoot<AccountId>;
    type MaxRemoteLockConsumers = ConstU32<0>;
    type RemoteLockConsumerIdentifier = ();
}

// TODO: restore tests for new configs
