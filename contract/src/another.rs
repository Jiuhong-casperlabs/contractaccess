#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{collections::BTreeMap, string::String, vec};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    runtime_args, CLType, ContractHash, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints,
    Key, RuntimeArgs,
};

pub const GROUP_UREF_NAME: &str = "group_uref";
pub const DEFAULT_MIN_AMOUNT: u128 = 10000000000;

#[no_mangle]
pub extern "C" fn call_access_other() {
    let contract_hash_str: String = runtime::get_named_arg("main_contract");
    let contract_hash = ContractHash::from_formatted_str(&contract_hash_str).unwrap();

    let _: () = runtime::call_contract(contract_hash, "test2", runtime_args! {});
}

#[no_mangle]
pub extern "C" fn call() {
    let named_keys: BTreeMap<String, Key> = BTreeMap::new();

    let mut entry_points = EntryPoints::new();

    let entry_point_1 = EntryPoint::new(
        String::from("call_access_other"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(entry_point_1);

    let (contract_hash, _) = storage::new_contract(entry_points, Some(named_keys), None, None);
    runtime::put_key("another_contract", contract_hash.into());
}
