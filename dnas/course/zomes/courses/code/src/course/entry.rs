use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use super::validation;

// Course struct contains all course data and is updated every time there are changes to the course
// NOTE (e-nastasia): Course doensn't have teacher_address field because only teacher can create
//      courses in every Course DNA (see course/validation.rs), so having this data here would be unneccessary
pub struct Course {
    title: String,
    modules: Vec<Address>,
    anchor_address: Address, // implicit link to CourseAnchor to be able to quickly retrieve the corresponding anchor
    timestamp: i32,
}

pub const COURSE_ENTRY_NAME: &str = "course";
pub const COURSE_ENTRY_DESCRIPTION: &str =
    "Course entry contains all course data and is updated on every course change";

// (e-nastasia) TODO: add a thoughtful description of why it is useful to provide this impl
impl HolochainEntry for Course {
    fn entry_type() -> String {
        String::from(COURSE_ENTRY_NAME)
    }
}

pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: COURSE_ENTRY_NAME,
        description: COURSE_ENTRY_DESCRIPTION,
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    validation::create(entry,validation_data)?
                },
                EntryValidationData::Modify { new_entry, old_entry,old_entry_header, validation_data } => {
                    validation::modify(new_entry,old_entry,old_entry_header,validation_data)?
                },
                EntryValidationData::Delete {old_entry,old_entry_header, validation_data } => {
                    validation::delete(old_entry, old_entry_header, validation_data)?
                }
            }
        },
        // NOTE (e-nastasia): Course entry relies on CourseAnchor to handle all linking and it already has
        //      CourseAnchor address saved inside it, so there's no need to define any other links
        links: [ ]
    )
}

// Anchor
// All_Course_Anchor
// e-nastasia: Q: if we have a course per DNA, maybe index for all courses should be moved from
//   the course DNA to course-catalog DNA? Otherwise we'll be creating "all courses anchor" per every Couse DNA
