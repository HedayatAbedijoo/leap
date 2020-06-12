use super::entry::Content;

use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::ValidationData;

pub fn create(_entry: Content, _validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn modify(
    _new_entry: Content,
    _old_entry: Content,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Ok(())
}

pub fn delete(
    _old_entry: Content,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Ok(())
}
