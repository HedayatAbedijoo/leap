use super::entry::{Module, MODULE_TO_CONTENT_LINK};
use crate::course;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

pub fn create(title: String, course_address: &Address, timestamp: u64) -> ZomeApiResult<Address> {
    //add new module to DHT
    let new_module = Module::new(title, timestamp);
    let new_module_address = hdk::commit_entry(&new_module.entry())?;

    course::handlers::add_new_module(&course_address, &new_module_address)?;
    Ok(new_module_address)
}

pub fn update(title: String, module_address: &Address) -> ZomeApiResult<Address> {
    let mut module: Module = hdk::utils::get_as_type(module_address.clone())?;
    module.update(title);
    hdk::update_entry(module.entry(), module_address)
}

pub fn delete(module_address: Address, course_address: Address) -> ZomeApiResult<Address> {
    let result = hdk::remove_entry(&module_address)?;
    course::handlers::remove_module(&course_address, &module_address)?;
    Ok(result)
}

pub fn link_new_content_to_module(
    module_address: Address,
    content_address: Address,
) -> ZomeApiResult<Address> {
    hdk::link_entries(
        &module_address,
        &content_address,
        MODULE_TO_CONTENT_LINK,
        "",
    )
}

pub fn remove_content_link_from_module(
    module_address: Address,
    content_address: Address,
) -> ZomeApiResult<()> {
    hdk::remove_link(
        &module_address,
        &content_address,
        MODULE_TO_CONTENT_LINK,
        "",
    )
}
