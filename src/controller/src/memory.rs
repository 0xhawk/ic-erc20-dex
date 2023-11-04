use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::DefaultMemoryImpl;
use ic_stable_structures::{StableCell};
use rustic::types::{Cbor, RM};
use std::cell::RefCell;

type Amount = u64;

#[derive(Clone, CandidType, serde::Serialize, serde::Deserialize, Default)]
pub struct CkicpState {
    pub tecdsa_pubkey: Vec<u8>,
    pub tecdsa_signer_address: [u8; 20],
    pub total_icp_locked: Amount,
    pub last_block: u64,
    pub next_blocks: std::collections::VecDeque<u64>,
}

const CKICP_CONFIG_SIZE: u64 = 512;
const CKICP_STATE_SIZE: u64 = 256;

const CKICP_CONFIG_PAGE_START: u64 = rustic::memory_map::USER_PAGE_START;
const CKICP_CONFIG_PAGE_END: u64 = CKICP_CONFIG_PAGE_START + CKICP_CONFIG_SIZE;
const CKICP_STATE_PAGE_START: u64 = CKICP_CONFIG_PAGE_END;
const CKICP_STATE_PAGE_END: u64 = CKICP_STATE_PAGE_START + CKICP_STATE_SIZE;

thread_local! {
    pub static CKICP_STATE: RefCell<StableCell<Cbor<Option<CkicpState>>, RM>> = RefCell::new(StableCell::init(
        RM::new(DefaultMemoryImpl::default(), CKICP_STATE_PAGE_START..CKICP_STATE_PAGE_END),
        Cbor::default(),
    ).expect("failed to initialize"));
}