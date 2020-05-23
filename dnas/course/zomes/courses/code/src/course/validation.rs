use hdk::holochain_persistence_api::cas::content::Address;

use strings::*;

pub fn validate_only_teacher_can_do(
    validation_data_sources: Vec<Address>,
    teacher_addr: &Address,
    action_name: &str,
) -> Result<(), String> {
    if !validation_data_sources.contains(teacher_addr) {
        return Err(format!(
            ERR_ONLY_TEACHER_CAN,
            action_name
        ));
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