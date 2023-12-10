use candid::{candid_method, CandidType};
use ic_cdk_macros::{export_candid, update};

pub type TxReceipt = Result<u16, TxError>;

#[derive(CandidType, Debug, PartialEq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other(String),
}

#[update(name = "transferFrom")]
#[candid_method(update, rename = "transferFrom")]
async fn transfer_from() -> TxReceipt {
    Ok(666)
}

export_candid!();
