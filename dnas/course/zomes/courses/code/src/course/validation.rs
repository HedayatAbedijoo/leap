//use super::anchor::CourseAnchor;
use super::entry::Course;

use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::ValidationData;
use hdk::LinkValidationData;

// ========== Course entry validation ========

pub fn create(validation_data: ValidationData) -> Result<(), String> {
    crate::helper::validate_only_teacher_can_do(validation_data.sources(), "create courses")
}

pub fn modify(
    _new_entry: Course,
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    crate::helper::validate_only_teacher_can_do(validation_data.sources(), "modify courses")
}

pub fn _delete(
    _old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    crate::helper::validate_only_teacher_can_do(validation_data.sources(), "delete courses")
}

// ========== Course anchor validation ========

pub fn validate_anchor(validation_data: ValidationData) -> Result<(), String> {
    crate::helper::validate_only_teacher_can_do(validation_data.sources(), "create anchor")
}

pub fn validate_anchor_link(validation_data: LinkValidationData) -> Result<(), String> {
    match validation_data {
        hdk::LinkValidationData::LinkAdd { link: _, validation_data, } => {
            crate::helper::validate_only_teacher_can_do(validation_data.sources(), "create this link")
        },
        hdk::LinkValidationData::LinkRemove { link: _, validation_data, } => {
            crate::helper::validate_only_teacher_can_do(validation_data.sources(), "remove this link")
        },
    }
}
