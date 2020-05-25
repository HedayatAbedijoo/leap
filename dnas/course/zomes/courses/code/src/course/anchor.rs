use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use super::validation;

// CourseAnchor provides a constant address for a course to be referenced by.
// Once created, it never changes. It is linked to all Course entries to provide access to the course data.
pub struct CourseAnchor {
    timestamp: i32,
    teacher_address: Address,
}

pub const COURSE_ANCHOR_ENTRY_NAME: &str = "course_anchor";
pub const COURSE_ANCHOR_ENTRY_DESCRIPTION: &str =
    "CourseAnchor entry provides constant address for a course to be referenced by";
pub const LINK_COURSE_ANCHOR_TO_COURSE: &str = "course_anchor->course";
pub const LINK_TEACHER_TO_COURSE_ANCHOR: &str = "teacher->courses";
pub const LINK_STUDENT_TO_COURSE_ANCHOR: &str = "student->courses";
pub const LINK_COURSE_ANCHOR_TO_STUDENT: &str = "course->students";

impl HolochainEntry for CourseAnchor {
    fn entry_type() -> String {
        String::from(COURSE_ANCHOR_ENTRY_NAME)
    }
}

pub fn course_anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: COURSE_ANCHOR_ENTRY_NAME,
        description: COURSE_ANCHOR_ENTRY_DESCRIPTION,
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<CourseAnchor>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    // NOTE (e-nastasia): these calls are identical to validation calls in course/entry.rs
                    //      but there are methods with different signature matching CourseAnchor type
                    validation::create(entry,validation_data)?
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    validation::modify(new_entry,old_entry,old_entry_header,validation_data)?
                },
                EntryValidationData::Delete {old_entry, validation_data, .. } => {
                    validation::delete(old_entry, old_entry_header, validation_data)?
                }
            }
        },
        links: [
            to!( // to query this course's latest data
                COURSE_ANCHOR_ENTRY_NAME,
                link_type: LINK_COURSE_ANCHOR_TO_COURSE,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: validation::link_course_anchor_to_course
            ),
            // (e-nastasia) NOTE: I don't think we'll need this link. In a single DNA there can only be courses that belong to user if
            //  their agent_address == DNA.teacher_address.
            //  And we don't have to answer this question for other DNAs located outside of the current one.
            // from!( // to query all the courses of a user(all courses that a user is the teacher or owner of)
            //     "%agent_id",
            //     link_type: LINK_TEACHER_TO_COURSE_ANCHOR,
            //     validation_package: || {
            //         hdk::ValidationPackageDefinition::Entry
            //     },
            //     validation: validation::link_teacher_to_course_anchor
            // ),
            // (e-nastasia) NOTE: in previous leap version we had link from "student->course" to allow listing all courses that someone is
            //      enrolled to. But now we have separate DNAs and list of all courses that someone is enrolled to would be calculated as:
            //      - get all course DNAs that agent is running
            //          - where DNA.teacher_address != agent_address
            //      So looks like we don't have to have this link anymore.
            //      As for link "course->student", this would be calculated as:
            //      - get all DHT participants
            //          - where agent_address != DNA.teacher_address
        ]
    )
}
