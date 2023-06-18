use crate::{
    permit::{caller_is_admin, caller_is_signer},
    wallet::version,
};
use b3_helper_lib::{revert, time::NanoTimeStamp, types::RequestId};
use b3_permit_lib::{
    pending::new::RequestArgs,
    request::{
        btc::transfer::BtcTransfer,
        global::SendToken,
        icp::transfer::IcpTransfer,
        inner::{
            account::{CreateAccount, RemoveAccount, RenameAccount},
            setting::UpdateCanisterSettings,
            signer::AddSigner,
        },
        request::{Request, RequestTrait},
    },
    signer::roles::Roles,
    store::{with_permit, with_permit_mut, with_signer_check, with_signer_ids_by_role},
    types::PendingRequestList,
};
use b3_wallet_lib::ledger::subaccount::SubaccountTrait;
use b3_wallet_lib::store::with_ledger;
use ic_cdk::{export::candid::candid_method, query, update};

// QUERY ---------------------------------------------------------------------

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
pub fn get_pending_list() -> PendingRequestList {
    with_permit(|s| s.pending_list())
}

#[candid_method(query)]
#[query(guard = "caller_is_signer")]
pub fn is_connected() -> bool {
    let caller = ic_cdk::caller();

    with_signer_check(caller, |signer| signer.is_canister()).is_ok()
}

// UPDATE ---------------------------------------------------------------------
#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn request_maker(
    request: Request,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_add_signer(
    request: AddSigner,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[update]
#[candid_method(update)]
pub fn request_connect() -> RequestId {
    let caller = ic_cdk::caller();

    let request = AddSigner {
        name: "B3 Peyment".to_string(),
        role: Roles::Canister,
        signer_id: caller,
        expires_at: None,
        threshold: None,
    };

    with_permit(|s| {
        // check if the request is already in the pending list
        let pending_list = s.pending_list();

        for pending_request in pending_list.iter() {
            if pending_request.request == Request::AddSigner(request.clone()) {
                return revert("Already Pending!");
            }
        }
    });

    if with_signer_check(caller, |signer| signer.is_canister()).is_ok() {
        return revert("Already connected!");
    }

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role.clone(), |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason: format!("Connecting to B3Payment for making payment!"),
        deadline: None,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_update_settings(
    request: UpdateCanisterSettings,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    request.validate_request().unwrap_or_else(revert);

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_account_rename(
    request: RenameAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_create_account(
    request: CreateAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_delete_account(
    request: RemoveAccount,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_transfer_icp(
    request: IcpTransfer,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_admin")]
pub fn request_transfer_btc(
    request: BtcTransfer,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub fn request_send(
    request: SendToken,
    reason: String,
    deadline: Option<NanoTimeStamp>,
) -> RequestId {
    let caller = ic_cdk::caller();

    let role = Roles::Admin;
    let allowed_signers = with_signer_ids_by_role(role, |signer_ids| signer_ids.to_vec());

    let request_args = RequestArgs {
        allowed_signers,
        role,
        request: request.into(),
        version: version(),
        reason,
        deadline,
    };

    with_permit_mut(|s| {
        let new_request = s.new_request(caller, request_args);
        s.insert_new_request(new_request)
    })
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_sign_message(account_id: String, message_hash: Vec<u8>) -> Vec<u8> {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    ledger
        .subaccount
        .sign_with_ecdsa(message_hash)
        .await
        .unwrap_or_else(revert)
}

#[candid_method(update)]
#[update(guard = "caller_is_signer")]
pub async fn request_sign_transaction(
    account_id: String,
    hex_raw_tx: Vec<u8>,
    chain_id: u64,
) -> Vec<u8> {
    let ledger = with_ledger(&account_id, |ledger| ledger.clone()).unwrap_or_else(revert);

    ledger
        .sign_evm_transaction(hex_raw_tx, chain_id)
        .await
        .unwrap_or_else(revert)
}
