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
                validation:|validation_data: hdk::LinkValidationData|{
                   validation::validate_anchor_link(validation_data)
                }
            )
        ]
    )
}
