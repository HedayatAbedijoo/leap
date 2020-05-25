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
        validation: | validation_data: hdk::EntryValidationData<Course>| {
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
            from!( // to query all the courses of a user(all courses that a user is the teacher or owner of)
                "%agent_id",
                link_type: LINK_TEACHER_TO_COURSE_ANCHOR,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: validation::link_teacher_to_course_anchor
            ),
            // e-nastasia: Q: would this link work & make sense with DNA-per-course architecture? Because with it
            //       every course that certain agent is enrolled to is just another DNA that this agent is running. Of course,
            //       we still need to differentiate between courses where agent is a teacher vs courses where agent is a student
            from!( // to query all courses that one user enrolled
                "%agent_id",
                link_type: LINK_STUDENT_TO_COURSE_ANCHOR,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            // e-nastasia: Q: pretty much the same applies here. This link is in an opposite direction, but it's usefullness is still
            //      questionable in DNA-per-course architecture
            to!( // to query all enrolled user for a course
                "%agent_id",
                link_type: LINK_COURSE_ANCHOR_TO_STUDENT,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    // TODO: validate that we're not linking to course teacher
                    Ok(())
                }
            )
        ]
    )
}
