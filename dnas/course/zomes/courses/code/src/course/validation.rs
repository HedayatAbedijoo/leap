use super::entry::Course;
use super::anchor::CourseAnchor;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::holochain_persistence_api::cas::content::Address;

use hdk::ValidationData;

// ========== Course validation ========

pub fn create(entry: Course, validation_data: ValidationData) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "create")
}

pub fn modify(
    _new_entry: Course,
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "modify")
}

pub fn delete(
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "delete")
}

// ========== CourseAnchor validation ========

pub fn create(entry: CourseAnchor, validation_data: ValidationData) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "create")
}

pub fn modify(
    _new_entry: CourseAnchor,
    _old_entry: CourseAnchor,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    Err("You can not modify CourseAnchor".to_string())
}

pub fn delete(
    _old_entry: CourseAnchor,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "delete")
}

// ========== private validation helpers ========

fn validate_only_teacher_can_do(
    validation_data_sources: Vec<Address>,
    action_name: &str,
) -> Result<(), String> {
    if !validation_data_sources.contains(&crate::helper::get_teacher_address()?) {
        return Err(format!("Only the teacher can {} courses", action_name));
    }
    Ok(())
}

