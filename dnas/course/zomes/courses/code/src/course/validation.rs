//use super::anchor::CourseAnchor;
use super::entry::Course;

use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::ValidationData;

// ========== Course entry validation ========

pub fn modify(
    _new_entry: Course,
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "modify")
}

pub fn _delete(
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "delete")
}

pub fn create(validation_data: ValidationData) -> Result<(), String> {
    validate_only_teacher_can_do(validation_data.sources(), "Create")
}

pub fn validate_anchor(validation_data: ValidationData) -> Result<(), String> {
    if !validation_data
        .sources()
        .contains(&crate::helper::get_teacher_address()?)
    {
        return Err(format!("Only the teacher can create anchor"));
    }
    Ok(())
}

// ========== CourseAnchor links validation ========

// TODO:  should validate that we're only linking against Course that has
// this entry's address in course_anchor field
// pub fn link_course_anchor_to_course(
//     _validation_data: hdk::LinkValidationData,
// ) -> Result<(), String> {
//     Ok(())
// }

// // TODO: validate that we're only linking to courses that have teacher_address equal to %agent_id
// pub fn link_teacher_to_course_anchor(
//     _validation_data: hdk::LinkValidationData,
// ) -> Result<(), String> {
//     Ok(())
// }

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
