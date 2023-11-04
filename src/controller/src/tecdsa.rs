use candid::CandidType;
use candid::Principal;
use serde::{Deserialize, Serialize};
use ic_cdk::api::call::{CallResult};
use ic_cdk::call;
use lazy_static::lazy_static;

#[derive(CandidType, Deserialize, Debug)]
pub struct ECDSAPublicKeyReply {
    pub public_key: Vec<u8>,
    pub chain_code: Vec<u8>,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EcdsaCurve {
    #[serde(rename = "secp256k1")]
    Secp256k1,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EcdsaKeyId {
    pub curve: EcdsaCurve,
    pub name: String,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct ECDSAPublicKey {
    pub canister_id: Option<Principal>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: EcdsaKeyId,
}

#[derive(CandidType, Serialize, Debug)]
pub struct ManagementCanister {}

lazy_static! {
    pub static ref MGMT_ID: Principal = Principal::from_text("aaaaa-aa").unwrap();
}

impl ManagementCanister {
    pub async fn ecdsa_public_key(key_name: &str, canister_id: Principal) -> CallResult<(ECDSAPublicKeyReply,)> {
        let key_id = EcdsaKeyId { 
            curve: EcdsaCurve::Secp256k1,
            name: key_name.to_string(),
        };
        let request = ECDSAPublicKey {
            canister_id: Some(canister_id),
            derivation_path: vec![],
            key_id: key_id.clone(),
        };
        call(*MGMT_ID, "ecdsa_public_key", (request,)).await
    }
}
