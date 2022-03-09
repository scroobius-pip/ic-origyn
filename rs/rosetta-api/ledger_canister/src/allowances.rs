use candid::{candid_method, CandidType, Deserialize, Int, Nat};
use cap_sdk::{handshake, insert, Event, IndefiniteEvent, TypedEvent};
use cap_std::dip20::cap::DIP20Details;
use cap_std::dip20::{Operation, TransactionStatus, TxRecord};
use ic_cdk_macros::*;
use ic_kit::{ic, Principal};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::Into;
use std::iter::FromIterator;
use std::string::String;

#[derive(CandidType, Default, Deserialize, Clone)]
pub struct TxLog {
    pub ie_records: VecDeque<IndefiniteEvent>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, CandidType, Clone, Debug)]
struct Metadata {
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    totalSupply: Nat,
    owner: Principal,
    fee: Nat,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
struct StatsData {
    logo: String,
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: Nat,
    owner: Principal,
    fee: Nat,
    fee_to: Principal,
    history_size: usize,
    deploy_time: u64,
}

#[allow(non_snake_case)]
#[derive(Deserialize, CandidType, Clone, Debug)]
struct TokenInfo {
    metadata: Metadata,
    feeTo: Principal,
    // status info
    historySize: usize,
    deployTime: u64,
    holderNumber: usize,
    cycles: u64,
}

impl Default for StatsData {
    fn default() -> Self {
        StatsData {
            logo: "".to_string(),
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0u8,
            total_supply: Nat::from(0),
            owner: Principal::anonymous(),
            fee: Nat::from(0),
            fee_to: Principal::anonymous(),
            history_size: 0,
            deploy_time: 0,
        }
    }
}

type Balances = HashMap<Principal, Nat>;
type Allowances = HashMap<Principal, HashMap<Principal, Nat>>;

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
pub type TxReceipt = Result<Nat, TxError>;

thread_local! {
    static ALLOWS: RefCell<HashMap<Principal, HashMap<Principal, Nat>>> = RefCell::new(HashMap::default());
}

fn _transfer(from: Principal, to: Principal, value: Nat) {
    // BALANCES.with(|b| {
    //     let mut balances = b.borrow_mut();
    //     let from_balance = balance_of(from);
    //     let from_balance_new: Nat = from_balance - value.clone();
    //     if from_balance_new != 0 {
    //         balances.insert(from, from_balance_new);
    //     } else {
    //         balances.remove(&from);
    //     }
    //     let to_balance = balance_of(to);
    //     let to_balance_new = to_balance + value;
    //     if to_balance_new != 0 {
    //         balances.insert(to, to_balance_new);
    //     }
    // });
}

fn _charge_fee(user: Principal, fee: Nat) {
    // STATS.with(|s| {
    //     let stats = s.borrow();
    //     if stats.fee > Nat::from(0) {
    //         _transfer(user, stats.fee_to, fee);
    //     }
    // });
}

fn _get_fee() -> Nat {
    // STATS.with(|s| {
    //     let stats = s.borrow();
    //     stats.fee.clone()
    // })
}

fn _get_owner() -> Principal {
    // STATS.with(|s| {
    //     let stats = s.borrow();
    //     stats.owner
    // })
}

fn _history_inc() {
    // STATS.with(|s| {
    //     let mut stats = s.borrow_mut();
    //     stats.history_size += 1;
    // })
}

#[update(name = "transferFrom")]
#[candid_method(update, rename = "transferFrom")]
async fn transfer_from(from: Principal, to: Principal, value: Nat) -> TxReceipt {

    ALLOWS.with(|a| {
        let mut allowances = a.borrow_mut();
        match allowances.get(&from) {
            Some(inner) => {
                let result = inner.get(&owner).unwrap().clone();
                let mut temp = inner.clone();
                if result.clone() - value.clone() - fee.clone() != 0 {
                    temp.insert(owner, result.clone() - value.clone() - fee.clone());
                    allowances.insert(from, temp);
                } else {
                    temp.remove(&owner);
                    if temp.len() == 0 {
                        allowances.remove(&from);
                    } else {
                        allowances.insert(from, temp);
                    }
                }
            }
            None => {
                assert!(false);
            }
        }
    });
}

async fn approve(spender: Principal, value: Nat) -> TxReceipt {
    let owner = ic::caller();
    let fee = _get_fee();
    if balance_of(owner) < fee.clone() {
        return Err(TxError::InsufficientBalance);
    }
    _charge_fee(owner, fee.clone());
    let v = value.clone() + fee.clone();
    ALLOWS.with(|a| {
        let mut allowances = a.borrow_mut();
        match allowances.get(&owner) {
            Some(inner) => {
                let mut temp = inner.clone();
                if v.clone() != 0 {
                    temp.insert(spender, v.clone());
                    allowances.insert(owner, temp);
                } else {
                    temp.remove(&spender);
                    if temp.len() == 0 {
                        allowances.remove(&owner);
                    } else {
                        allowances.insert(owner, temp);
                    }
                }
            }
            None => {
                if v.clone() != 0 {
                    let mut inner = HashMap::new();
                    inner.insert(spender, v.clone());
                    let allowances = ic::get_mut::<Allowances>();
                    allowances.insert(owner, inner);
                }
            }
        }
    });

    _history_inc();
    add_record(
        owner,
        Operation::Approve,
        owner,
        spender,
        v,
        fee.clone(),
        ic::time(),
        TransactionStatus::Succeeded,
    )
        .await
}

fn allowance(owner: Principal, spender: Principal) -> Nat {
    ALLOWS.with(|a| {
        let allowances = a.borrow();
        match allowances.get(&owner) {
            Some(inner) => match inner.get(&spender) {
                Some(value) => value.clone(),
                None => Nat::from(0),
            },
            None => Nat::from(0),
        }
    })
}

fn get_allowance_size() -> usize {
    let mut size = 0;
    ALLOWS.with(|a| {
        let allowances = a.borrow();
        for (_, v) in allowances.iter() {
            size += v.len();
        }
        size
    })
}

fn get_user_approvals(who: Principal) -> Vec<(Principal, Nat)> {
    ALLOWS.with(|a| {
        let allowances = a.borrow();
        match allowances.get(&who) {
            Some(allow) => Vec::from_iter(allow.clone().into_iter()),
            None => Vec::new(),
        }
    })
}

#[pre_upgrade]
fn pre_upgrade() {
    let allows = ALLOWS.with(|a| a.borrow().clone());
    ic::stable_store(allows).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    let allowances_stored: Allowances = ic::stable_restore().unwrap();
    ALLOWS.with(|a| {
        let mut allowances = a.borrow_mut();
        *allowances = allowances_stored;
    });
}