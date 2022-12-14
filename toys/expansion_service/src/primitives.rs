// Copyright (c) Developer.
// SPDX-License-Identifier: Apache-2.0

use proc_macro_crate::ExactSuiJsonValue;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, ExactSuiJsonValue)]
pub struct CreateSceneParameter {
    pub power: u64,
    pub radius: u64,
    pub equilibrium: u64,
    pub frames: u64,
    pub frame_interval: u64,
    pub next_frame_block: u64,
    pub max_participant: u64,
    pub min_stake_amount: u64,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct XCoinObject {}

#[derive(Debug, Clone, Serialize, ExactSuiJsonValue)]
pub struct CoinMintParameter {
    pub object_id: String,
    pub amount: u64,
    pub recipient: String,
}

#[derive(Debug, Clone, Serialize, ExactSuiJsonValue)]
pub struct EnterParameter {
    pub scene_object_id: String,
    pub stake_xcoin_id: String,
    pub participant: String,
}
