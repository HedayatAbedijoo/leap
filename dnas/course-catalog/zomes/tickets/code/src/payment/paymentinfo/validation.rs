use super::entry::PaymentInfo;
use hdk::holochain_core_types::chain_header::ChainHeader;

use hdk::ValidationData;

pub fn create(entry: PaymentInfo, validation_data: ValidationData) -> Result<(), String> {
    hdk::debug(format!("entry: {:?}", entry)).ok();
    hdk::debug(format!("validation_data: {:?}", validation_data)).ok();

    Err("Non Implemented Validation Rules".to_string())
}

pub fn modify(
    _new_entry: PaymentInfo,
    _old_entry: PaymentInfo,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Err("Non Implemented Validation Rules".to_string())
}

pub fn delete(
    _old_entry: PaymentInfo,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Err("Non Implemented Validation Rules".to_string())
}
