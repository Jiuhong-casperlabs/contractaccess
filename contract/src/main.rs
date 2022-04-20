#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use alloc::{
    collections::BTreeMap,
    format,
    string::{String, ToString},
    vec,
};

use casper_contract::contract_api::{runtime, storage};
use casper_types::{
    runtime_args, system::CallStackElement, CLType, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Group, Key, RuntimeArgs,
};

pub const GROUP_LABEL: &str = "group_label";
pub const GROUP_UREF_NAME: &str = "group_uref";
pub const DEFAULT_MIN_AMOUNT: u128 = 10000000000;

#[no_mangle]
pub extern "C" fn test1() {
    let stacks = runtime::get_call_stack();

    for (i, el) in stacks.iter().enumerate() {
        let key_name = format!("stack_ {}!", i.to_string());
        match el {
            CallStackElement::Session { account_hash } => {
                runtime::put_key(&key_name, (*account_hash).into())
            }
            CallStackElement::StoredSession {
                account_hash,
                contract_hash,
                contract_package_hash,
            } => {
                let mut map: BTreeMap<String, Key> = BTreeMap::new();
                let key1 = String::from("account_hash_session");
                let key2 = String::from("contract_package_hash_session");
                let key3 = String::from("contract_hash_session");

                //store purse into contract named_keys
                map.insert(key1, (*account_hash).into());
                map.insert(key2, (*contract_package_hash).into());
                map.insert(key3, (*contract_hash).into());
                runtime::put_key(&key_name, storage::new_uref(map).into());
            }
            CallStackElement::StoredContract {
                contract_hash,
                contract_package_hash,
            } => {
                let mut map: BTreeMap<String, Key> = BTreeMap::new();
                let key2 = String::from("contract_package_hash_session");
                let key3 = String::from("contract_hash_session");
                //store purse into contract named_keys
                map.insert(key2, (*contract_package_hash).into());
                map.insert(key3, (*contract_hash).into());
                runtime::put_key(&key_name, storage::new_uref(map).into());
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn test2() {
    let stacks = runtime::get_call_stack();

    for (i, el) in stacks.iter().enumerate() {
        let key_name = format!("stack_ {}!", i.to_string());
        match el {
            CallStackElement::Session { account_hash } => {
                runtime::put_key(&key_name, (*account_hash).into())
            }
            CallStackElement::StoredSession {
                account_hash,
                contract_hash,
                contract_package_hash,
            } => {
                let mut map: BTreeMap<String, Key> = BTreeMap::new();
                let key1 = String::from("account_hash_session");
                let key2 = String::from("contract_package_hash_session");
                let key3 = String::from("contract_hash_session");

                //store purse into contract named_keys
                map.insert(key1, (*account_hash).into());
                map.insert(key2, (*contract_package_hash).into());
                map.insert(key3, (*contract_hash).into());
                runtime::put_key(&key_name, storage::new_uref(map).into());
            }
            CallStackElement::StoredContract {
                contract_hash,
                contract_package_hash,
            } => {
                let mut map: BTreeMap<String, Key> = BTreeMap::new();
                let key2 = String::from("contract_package_hash_session");
                let key3 = String::from("contract_hash_session");
                //store purse into contract named_keys
                map.insert(key2, (*contract_package_hash).into());
                map.insert(key3, (*contract_hash).into());
                runtime::put_key(&key_name, storage::new_uref(map).into());
            }
        }
    }
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
