use super::entry::Catalog;
use hdk::holochain_core_types::chain_header::ChainHeader;

use hdk::ValidationData;

pub fn create(entry: Catalog, validation_data: ValidationData) -> Result<(), String> {
    if validation_data.sources().contains(&entry.teacher_address) {
        Err("You can not create a Catalog for others".to_string())
    } else {
        Ok(())
    }
}

pub fn modify(
    _new_entry: Catalog,
    _old_entry: Catalog,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Err("You can not edit entry".to_string())
}

pub fn delete(
    _old_entry: Catalog,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Err("You can not delete entry".to_string())
}
