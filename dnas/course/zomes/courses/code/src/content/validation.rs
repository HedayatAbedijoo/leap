use super::entry::Content;

use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::ValidationData;

pub fn create(entry: Content, validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn modify(
    _new_entry: Content,
    _old_entry: Content,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    Ok(())
}

pub fn delete(
    _old_entry: Content,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    Ok(())
}

