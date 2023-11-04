#![allow(unused_imports)]

use ic_cdk_macros::{init, post_upgrade, pre_upgrade, query, update};

use controller::memory::*;
use controller::tecdsa::{ECDSAPublicKeyReply};
use controller::utils::*;
// use rustic::access_control::*;
// use rustic::inter_canister::*;
// use rustic::memory_map::*;
// use rustic::reentrancy_guard::*;
// use rustic::types::*;
// use rustic::utils::*;
// use rustic_macros::modifiers;

#[init]
pub fn init() {
    rustic::rustic_init();
}

#[query]
pub fn get_ckipc_state() -> CkicpState {
    CKICP_STATE.with(|state| {
        let ckicp_state = state.borrow();
        ckicp_state.get().0.clone().unwrap_or_default()
    })
}

#[query]
pub fn get_tecdsa_signer_address_hex() -> String {
    let state: CkicpState = get_ckipc_state();
    hex_encode(&state.tecdsa_signer_address)
}

fn main() {
}

#[cfg(any(target_arch = "wasm32", test))]
ic_cdk::export_candid!();