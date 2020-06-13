use super::{entry::Course, validation};

use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::convert::TryFrom;

// Since there would be one course inside the whole DNA, Just one empy Anchor is enough
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TheCourseAnchor {}

impl AnchorTrait for TheCourseAnchor {
    fn entry_type() -> String {
        String::from("the_course")
    }
    fn link_to() -> String {
        Course::entry_type()
    }
    fn link_type() -> String {
        "SELECT_Course_Of_DNA".to_string()
    }
}

pub trait AnchorTrait: TryFrom<JsonString> + Into<JsonString> + Clone {
    fn entry_type() -> String;
    fn link_to() -> String;
    fn link_type() -> String;
    fn entry(self) -> Entry {
        Entry::App(Self::entry_type().into(), self.into())
    }

    fn address(&self) -> ZomeApiResult<Address> {
        let addrss = hdk::commit_entry(&self.clone().entry())?;
        return Ok(addrss);
    }
}
pub fn the_course_anchor_def() -> ValidatingEntryType {
    entry!(
        name: TheCourseAnchor::entry_type(),
        description: "Anchor to the valid Course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<TheCourseAnchor>| {
            match validation_data{
                EntryValidationData::Create { validation_data,.. } => {
                    validation::validate_anchor(validation_data)?;
                     Ok(())
                 },
                 EntryValidationData::Modify { .. } => {
                    return Err(String::from("Cannot modify the anchor"));
                 },
                 EntryValidationData::Delete {.. } => {
                    return Err(String::from("Cannot modify the anchor"));
                 }
            }
        },
        links:[
            to!(
                TheCourseAnchor::link_to(),
                link_type: TheCourseAnchor::link_type(),
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                   Ok(()) //TODO: Validate only Teacher should be able to create and modify link
                }
            )
        ]
    )
}
// pub fn the_course_anchor_def() -> ValidatingEntryType {
//     entry!(
//         name: TheCourseAnchor::entry_type(),
//         description:"Anchor to the valid Course",
//         sharing: Sharing::Public,
//         validation_package:||{
//             hdk::ValidationPackageDefinition::Entry
//         },
//         validation: | validation_data: hdk::EntryValidationData<TheCourseAnchor>| {
//             match validation_data{
//                 EntryValidationData::Create { validation_data,.. } => {
//                     validation::validate_anchor(validation_data)?;
//                      Ok(())
//                  },
//                  EntryValidationData::Modify { validation_data,.. } => {
//                     validation::validate_anchor(validation_data)?;
//                      Ok(())
//                     //return Err(String::from("Cannot modify the anchor"));
//                  },
//                  EntryValidationData::Delete {validation_data,.. } => {
//                     validation::validate_anchor(validation_data)?;
//                     Ok(())
//                     // return Err(String::from("Cannot delete the anchor"));
//                  }
//             }
//         },
//         links:[
//             to!(
//                 TheCourseAnchor::link_to(),
//                 link_type: TheCourseAnchor::link_type(),
//                 validation_package:||{
//                     hdk::ValidationPackageDefinition::Entry
//                 },
//                 validation:|validation_data: hdk::LinkValidationData|{
//                    Ok(()) //TODO: Validate only Teacher should be able to create and modify link
//                 }
//             )
//         ]
//     )
// }
// use hdk::holochain_persistence_api::cas::content::Address;
// use hdk::{
//     entry_definition::ValidatingEntryType,
//     error::{ZomeApiError, ZomeApiResult},
//     AGENT_ADDRESS,
// };

// use super::validation;

// // CourseAnchor provides a constant address for a course to be referenced by.
// // Once created, it never changes. It is linked to all Course entries to provide access to the course data.
// pub struct CourseAnchor {
//     timestamp: i32,
//     course_title: String,
// }

// pub const COURSE_ANCHOR_ENTRY_NAME: &str = "course_anchor";
// pub const COURSE_ANCHOR_ENTRY_DESCRIPTION: &str =
//     "CourseAnchor entry provides constant address for a course to be referenced by";
// pub const LINK_COURSE_ANCHOR_TO_COURSE: &str = "course_anchor->course";

// impl HolochainEntry for CourseAnchor {
//     fn entry_type() -> String {
//         String::from(COURSE_ANCHOR_ENTRY_NAME)
//     }
// }

// pub fn course_anchor_entry_def() -> ValidatingEntryType {
//     entry!(
//         name: COURSE_ANCHOR_ENTRY_NAME,
//         description: COURSE_ANCHOR_ENTRY_DESCRIPTION,
//         sharing: Sharing::Public,
//         validation_package: || {
//             hdk::ValidationPackageDefinition::Entry
//         },
//         validation: | validation_data: hdk::EntryValidationData<CourseAnchor>| {
//             match validation_data{
//                 EntryValidationData::Create { entry, validation_data } => {
//                     // NOTE (e-nastasia): these calls are identical to validation calls in course/entry.rs
//                     //      but there are methods with different signature matching CourseAnchor type
//                     validation::create(entry,validation_data)?
//                 },
//                 EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
//                     validation::modify(new_entry,old_entry,old_entry_header,validation_data)?
//                 },
//                 EntryValidationData::Delete {old_entry, validation_data, .. } => {
//                     validation::delete(old_entry, old_entry_header, validation_data)?
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
//                 validation: validation::link_course_anchor_to_course
//             ),
//             // NOTE (e-nastasia): we'll have at least one more link to anchor for indexing courses,
//             //       but this isn't yet defined
//         ]
//     )
// }
