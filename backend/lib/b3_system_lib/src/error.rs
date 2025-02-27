use b3_utils::error::HelperError;
use candid::{CandidType, Deserialize};

#[rustfmt::skip]
#[derive(CandidType, Deserialize)]
pub enum SystemError {
    HelperError(HelperError),
    InvalidSigner,
    ValidateSignerError(String),
    UpdateCanisterControllersError(String),
    VersionError(String),
    RateLimitExceeded,
    InvalidReleaseName(String),
    InvalidWalletCanister,
    InvalidAccountIdentifier,
    ReleaseNotFound,
    ReleaseNameNotFound,
    ReleaseAlreadyExists,
    WasmNotFound,
    WasmAlreadyLoaded,
    UserAlreadyExists,
    NoCanisterAvailable,
    UserNotFound,
    BugNotFound,
    BugsNotFound,
    OwnerMismatch { owner: String, user: String },
    UpdateControllersError(String),
    InstallArgError(String),
    EncodeError(String),
    WasmGetError(String),
    WasmHashError(String),
    InstallCodeError(String),
    WasmInstallError(String),
    WalletCanisterNotFound,
    WalletCanisterAlreadyInstalled,
    WalletCanisterRateError(String),
    WalletCanisterDoesNotExist(String),
    WalletCanisterAlreadyExists(String),
    CreateCanisterError(String),
    CanisterStatusError(String),
    ProductAlreadyExists,
    CanisterIdNotFound,
}

use std::fmt;

#[rustfmt::skip]
impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemError::HelperError(e) => write!(f, "{}", e),
            SystemError::InvalidSigner => write!(f, "Invalid user!"),
            SystemError::ValidateSignerError(e) => write!(f, "Validate user error: {}", e),
            SystemError::UpdateCanisterControllersError(e) => write!(f, "Update canister controllers error: {}", e),
            SystemError::VersionError(e) => write!(f, "Version error: {}", e),
            SystemError::RateLimitExceeded => write!(f, "Rate limit exceeded!"),
            SystemError::InvalidWalletCanister => write!(f, "Invalid wallet canister!"),
            SystemError::OwnerMismatch { owner, user } => write!(f, "Owner mismatch: {} != {}", owner, user),
            SystemError::InstallArgError(e) => write!(f, "Install arg error: {}", e),
            SystemError::UpdateControllersError(e) => write!(f, "Update controllers error: {}", e),
            SystemError::WasmInstallError(e) => write!(f, "Wasm install error: {}", e),
            SystemError::InvalidReleaseName(e) => write!(f, "Invalid release name: {}", e),
            SystemError::InvalidAccountIdentifier => write!(f, "Invalid account identifier!"),
            SystemError::ReleaseNotFound => write!(f, "Release not found!"),
            SystemError::ReleaseNameNotFound => write!(f, "Release name not found!"),
            SystemError::UserAlreadyExists => write!(f, "User already exists!"),
            SystemError::BugsNotFound => write!(f, "Bugs not found!"),
            SystemError::BugNotFound => write!(f, "Bug not found!"),
            SystemError::UserNotFound => write!(f, "User not found!"),
            SystemError::NoCanisterAvailable => write!(f, "No canister available!"),
            SystemError::ReleaseAlreadyExists => write!(f, "Release already exists!"),
            SystemError::WasmNotFound => write!(f, "Wasm not found!"),
            SystemError::WasmAlreadyLoaded => write!(f, "Wasm already loaded!"),
            SystemError::WasmGetError(e) => write!(f, "Wasm get error: {}", e),
            SystemError::WasmHashError(e) => write!(f, "Wasm hash error: {}", e),
            SystemError::EncodeError(e) => write!(f, "Encode error: {}", e),
            SystemError::InstallCodeError(e) => write!(f, "Install code error: {}", e),
            SystemError::CreateCanisterError(e) => write!(f, "Create canister error: {}", e),
            SystemError::CanisterStatusError(e) => write!(f, "Wallet status error: {}", e),
            SystemError::ProductAlreadyExists => write!(f, "Product already exists!"),
            SystemError::CanisterIdNotFound => write!(f, "Canister id not found!"),
            SystemError::WalletCanisterRateError(e) => write!(f, "Wallet canister rate error: {}", e),
            SystemError::WalletCanisterNotFound => write!(f, "Wallet Canister id not found!"),
            SystemError::WalletCanisterDoesNotExist(e) => write!(f, "Wallet does not exist: {}", e),
            SystemError::WalletCanisterAlreadyExists(e) => write!(f, "Wallet already exists: {}", e),
            SystemError::WalletCanisterAlreadyInstalled => write!(f, "Wallet canister already installed!"),
        }
    }
}
