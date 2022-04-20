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
    Group, Key, RuntimeArgs,
};

pub const GROUP_LABEL: &str = "group_label";
pub const GROUP_UREF_NAME: &str = "group_uref";
pub const DEFAULT_MIN_AMOUNT: u128 = 10000000000;

#[no_mangle]
pub extern "C" fn test1() {
    let a = runtime::get_call_stack();
    runtime::put_key("stacks in test1", storage::new_uref(a).into());
}

#[no_mangle]
pub extern "C" fn test2() {
    let a = runtime::get_call_stack();
    runtime::put_key("stacks in test2", storage::new_uref(a).into());
}

#[no_mangle]
pub extern "C" fn call() {
    let other_contract_str: String = runtime::get_named_arg("other_contract");
    let other_contract_hash = ContractHash::from_formatted_str(&other_contract_str).unwrap();

    let named_keys: BTreeMap<String, Key> = BTreeMap::new();

    let mut entry_points = EntryPoints::new();

    let entry_point_1 = EntryPoint::new(
        "test1",
        vec![],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let entry_point_2 = EntryPoint::new(
        String::from("test2"),
        vec![],
        CLType::Unit,
        EntryPointAccess::Groups(vec![Group::new(GROUP_LABEL)]),
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(entry_point_1);
    entry_points.add_entry_point(entry_point_2);

    // access - contract
    let (contract_package_hash, _access_uref) = storage::create_contract_package_at_hash();

    let admin_group = storage::create_contract_user_group(
        contract_package_hash,
        GROUP_LABEL,
        2,
        Default::default(),
    )
    .unwrap();

    runtime::put_key(GROUP_UREF_NAME, admin_group[0].into());

    let _: () = runtime::call_contract(
        other_contract_hash,
        "add_urefs",
        runtime_args! {"uref" => admin_group[1]},
    );

    let (contract_hash, _) =
        storage::add_contract_version(contract_package_hash, entry_points, named_keys);

    runtime::put_key("my_contract", contract_hash.into());
    runtime::put_key("my_contract_package", contract_package_hash.into());
}
