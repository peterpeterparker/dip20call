use candid::{CandidType, Principal};
use ic_cdk::api::call::CallResult;
use ic_cdk::{call, print};
use ic_cdk_macros::{export_candid, query, update};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug, PartialEq)]
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

pub type TxReceipt = Result<u16, TxError>;

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
async fn hello() {
    let call_result: CallResult<(TxReceipt,)> =
        call(Principal::from_text("be2us-64aaa-aaaaa-qaabq-cai".to_string()).unwrap(), "transferFrom", ()).await;

    print(format!("{:?}", call_result.unwrap().0));
}

export_candid!();
