use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use super::validation;
use strings::*;

// Course struct contains all course data and is updated every time there are changes to the course
pub struct Course {
    title: String,
    modules: Vec<Address>,
    anchor_address: Address, // implicit link to CourseAnchor to be able to quickly retrieve the corresponding anchor
    timestamp: i32,
}

// CourseAnchor provides a constant address for a course to be referenced by.
// Once created, it never changes. It is linked to all Course entries to provide access to the course data.
// pub struct CourseAnchor {
//     timestamp: i32,
//     teacher_address: Address,
// }
pub const COURSE_ENTRY_NAME: &str = "course";
pub const COURSE_ENTRY_DESCRIPTION: &str =
    "Course entry contains all course data and is updated on every course change";

// (e-nastasia) TODO: add a thoughtful description of why it is useful to provide this impl
impl HolochainEntry for Course {
    fn entry_type() -> String {
        String::from(COURSE_ENTRY_NAME)
    }
}

// impl HolochainEntry for CourseAnchor {
//     fn entry_type() -> String {
//         String::from(COURSE_ANCHOR_ENTRY_NAME)
//     }
// }

// ================================= HDK entry definitions

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
                   validation::create(entry,validation_data)?;
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry,old_entry_header, validation_data } => {
                    validation::modify( new_entry,old_entry,old_entry_header,validation_data,)?;
                    Ok(())
                },
                EntryValidationData::Delete {old_entry,old_entry_header, validation_data } => {
                 validation::delete(old_entry, old_entry_header, validation_data)?;
                    Ok(())
                }
            }
        },
        // NOTE: Course entry relies on CourseAnchor to handle all linking and it already has CourseAnchor address saved
        //      inside it, so there's no need to define any other links
        links: [ ]
    )
}

// pub fn course_anchor_entry_def() -> ValidatingEntryType {
//     entry!(
//         name: COURSE_ANCHOR_ENTRY_NAME,
//         description: COURSE_ANCHOR_ENTRY_DESCRIPTION,
//         sharing: Sharing::Public,
//         validation_package: || {
//             hdk::ValidationPackageDefinition::Entry
//         },
//         validation: | validation_data: hdk::EntryValidationData<Course>| {
//             match validation_data{
//                 EntryValidationData::Create { entry, validation_data } => {
//                     // e-nastasia: don't think there's anything to be validated for anchor creation
//                 },
//                 EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
//                     // e-nastasia: TODO: anchor can't be modified so we need to ensure that here
//                 },
//                 EntryValidationData::Delete {old_entry, validation_data, .. } => {
//                     // e-nastasia: validate that only course teacher can remove the anchor
//                 }
//             }
//         },
//         links: [
//             to!( // to query this course's latest data
//                 COURSE_ANCHOR_ENTRY_NAME,
//                 link_type: LINK_COURSE_ANCHOR_TO_COURSE,
//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::Entry
//                 },
//                 validation: | _validation_data: hdk::LinkValidationData | {
//                     // TODO:  should validate that we're only linking against Course that has
//                     // this entry's address in course_anchor field
//                     Ok(())
//                 }
//             ),
//             from!( // to query all the courses of a user(all courses that a user is the teacher or owner of)
//                 "%agent_id",
//                 link_type: LINK_TEACHER_TO_COURSE_ANCHOR,
//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::Entry
//                 },
//                 validation: | _validation_data: hdk::LinkValidationData | {
//                     // TODO: validate that we're only linking to courses that have teacher_address equal to %agent_id
//                     Ok(())
//                 }
//             ),
//             // e-nastasia: Q: would this link work & make sense with DNA-per-course architecture? Because with it
//             //       every course that certain agent is enrolled to is just another DNA that this agent is running. Of course,
//             //       we still need to differentiate between courses where agent is a teacher vs courses where agent is a student
//             from!( // to query all courses that one user enrolled
//                 "%agent_id",
//                 link_type: LINK_STUDENT_TO_COURSE_ANCHOR,
//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::Entry
//                 },
//                 validation: | _validation_data: hdk::LinkValidationData | {
//                     Ok(())
//                 }
//             ),
//             // e-nastasia: Q: pretty much the same applies here. This link is in an opposite direction, but it's usefullness is still
//             //      questionable in DNA-per-course architecture
//             to!( // to query all enrolled user for a course
//                 "%agent_id",
//                 link_type: LINK_COURSE_ANCHOR_TO_STUDENT,
//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::Entry
//                 },
//                 validation: | _validation_data: hdk::LinkValidationData | {
//                     // TODO: validate that we're not linking to course teacher
//                     Ok(())
//                 }
//             )
//         ]
//     )
// }

// Anchor
// All_Course_Anchor
// e-nastasia: Q: if we have a course per DNA, maybe index for all courses should be moved from
//   the course DNA to course-catalog DNA? Otherwise we'll be creating "all courses anchor" per every Couse DNA
