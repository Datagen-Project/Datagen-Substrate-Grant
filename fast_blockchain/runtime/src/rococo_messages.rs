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

//! Everything required to serve Westend <-> Rococo messages.

use super::Runtime;
use frame_support::weights::Weight;
use xcm::prelude::*;

// import trait from dependency module
use ::pallet_bridge_messages::WeightInfoExt as MessagesWeightInfoExt;
use ::pallet_bridge_parachains::WeightInfoExt as ParachainsWeightInfoExt;

impl MessagesWeightInfoExt for pallet_bridge_messages::WeightInfo<super::Runtime> {
    fn expected_extra_storage_proof_size() -> u32 {
        bp_bridge_hub_rococo::EXTRA_STORAGE_PROOF_SIZE
    }

    fn receive_messages_proof_overhead_from_runtime() -> Weight {
        pallet_bridge_relayers::WeightInfo::<Runtime>::receive_messages_proof_overhead_from_runtime(
        )
    }

    fn receive_messages_delivery_proof_overhead_from_runtime() -> Weight {
        pallet_bridge_relayers::WeightInfo::<Runtime>::receive_messages_delivery_proof_overhead_from_runtime()
    }
}

// TODO: Check if it is useful - if we keep the rococo parachain
impl ParachainsWeightInfoExt for pallet_bridge_parachains::WeightInfo<crate::Runtime> {
    fn expected_extra_storage_proof_size() -> u32 {
        bp_bridge_hub_rococo::EXTRA_STORAGE_PROOF_SIZE
    }
}

// TODO: see how to restore now them and if it is needed
// To push to Polkadot
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{RialtoGrandpaInstance, Runtime, WithRialtoMessagesInstance};

//     use bridge_runtime_common::{
//         assert_complete_bridge_types,
//         integrity::{
//             assert_complete_with_relay_chain_bridge_constants, check_message_lane_weights,
//             AssertChainConstants, AssertCompleteBridgeConstants,
//         },
//     };

//     #[test]
//     fn ensure_millau_message_lane_weights_are_correct() {
//         check_message_lane_weights::<bp_millau::Millau, Runtime, WithRialtoMessagesInstance>(
//             bp_rialto::EXTRA_STORAGE_PROOF_SIZE,
//             bp_millau::MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX,
//             bp_millau::MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX,
//             false,
//         );
//     }

//     #[test]
//     fn ensure_bridge_integrity() {
//         assert_complete_bridge_types!(
//             runtime: Runtime,
//             with_bridged_chain_grandpa_instance: RialtoGrandpaInstance,
//             with_bridged_chain_messages_instance: WithRialtoMessagesInstance,
//             this_chain: bp_millau::Millau,
//             bridged_chain: bp_rialto::Rialto,
//         );

//         assert_complete_with_relay_chain_bridge_constants::<
//             Runtime,
//             RialtoGrandpaInstance,
//             WithRialtoMessagesInstance,
//         >(AssertCompleteBridgeConstants {
//             this_chain_constants: AssertChainConstants {
//                 block_length: bp_millau::BlockLength::get(),
//                 block_weights: bp_millau::BlockWeights::get(),
//             },
//         });
//     }

//     #[test]
//     fn rialto_millau_bridge_identifier_did_not_changed() {
//         // there's nothing criminal if it is changed, but then thou need to fix it across
//         // all deployments scripts, alerts and so on
//         assert_eq!(
//             *Bridge::get().lane_id().as_ref(),
//             hex_literal::hex!("efed785b626e94da3969257012f506524bcec78867420e26ff8c55ddcdb0f7b7")
//                 .into(),
//         );
//     }
// }
