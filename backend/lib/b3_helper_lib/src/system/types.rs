use crate::{account::AccountIdentifier, timestamp::NanoTimeStamp, wallet::CanisterId};
use candid::CandidType;
use ic_cdk::api::management_canister::main::CanisterStatusResponse;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Serialize)]
pub struct SystemCanisterStatus {
    pub status_at: NanoTimeStamp,
    pub version: String,
    pub user_status: usize,
    pub canister_id: CanisterId,
    pub canister_status: CanisterStatusResponse,
}

#[derive(CandidType)]
pub struct ICPAccountBalanceArgs {
    pub account: AccountIdentifier,
}
