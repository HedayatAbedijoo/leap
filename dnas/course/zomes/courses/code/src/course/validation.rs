use super::entry::Course;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::holochain_persistence_api::cas::content::Address;

use hdk::ValidationData;
use strings::*;

pub fn create(entry: Course, validation_data: ValidationData) -> Result<(), String> {
    // you need to compare Signature of Entry with Teacher_Address of DNA. in this case there is no need to put Teacher_Address in all entries.
    // the same with Edit, Delete
    if validation_data
        .sources()
        .contains(&crate::helper::get_teacher_address()?)
    {
        Err("You are not the teacher of this DNA, so you can not create a course".to_string())
    } else {
        Ok(())
    }
}

pub fn modify(
    _new_entry: Course,
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Err("You can not edit payment info entry".to_string())
}

pub fn delete(
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    Err("You can not delete payment info entry".to_string())
}

pub fn validate_only_teacher_can_do(
    validation_data_sources: Vec<Address>,
    teacher_addr: &Address,
    action_name: &str,
) -> Result<(), String> {
    if !validation_data_sources.contains(teacher_addr) {
        return Err(format!(ERR_ONLY_TEACHER_CAN, action_name));
    }
    Ok(())
}

pub fn validate_no_teacher_change(
    old_teacher_addr: &Address,
    new_teacher_addr: &Address,
) -> Result<(), String> {
    if old_teacher_addr != new_teacher_addr {
        return Err(String::from(ERR_NO_TEACHER_CHANGE));
    }
    Ok(())
}
