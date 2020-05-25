use super::anchor::CourseAnchor;
use super::entry::Course;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::holochain_persistence_api::cas::content::Address;

use hdk::ValidationData;

// ========== Course entry validation ========

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

// ========== CourseAnchor entry validation ========

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

// ========== CourseAnchor links validation ========

// TODO:  should validate that we're only linking against Course that has
// this entry's address in course_anchor field
pub fn link_course_anchor_to_course(
    _validation_data: hdk::LinkValidationData,
) -> Result<(), String> {
    Ok(())
}

// TODO: validate that we're only linking to courses that have teacher_address equal to %agent_id
pub fn link_teacher_to_course_anchor(
    _validation_data: hdk::LinkValidationData,
) -> Result<(), String> {
    Ok(())
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
