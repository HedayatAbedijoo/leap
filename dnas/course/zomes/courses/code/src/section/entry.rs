use hdk::prelude::*;
use super::validation;
use crate::content::entry::Content;
use holochain_entry_utils::HolochainEntry;
pub const SECTION_TO_CONTENT_LINK: &str = "Section->contents";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    pub title: String,
    pub timestamp: u64,
}

impl HolochainEntry for Section {
    fn entry_type() -> String {
        String::from("Section")
    }
}

impl Section {
    pub fn new(title: String, timestamp: u64) -> Self {
        Section {
            title: title,
            timestamp: timestamp,
        }
    }

    pub fn update(&mut self, title: String) {
        self.title = title;
        self.timestamp += 1;
    }
}

pub fn section_entry_def() -> ValidatingEntryType {
    entry!(
        name: Section::entry_type(),
        description: "Section entry contains single Section from some course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Section>| {
            match validation_data{
                EntryValidationData::Create { validation_data,.. } => {
                    validation::validate_only_teacher_can_do(validation_data)?;
                    Ok(())
                },
                EntryValidationData::Modify { validation_data,.. } => {
                    validation::validate_only_teacher_can_do(validation_data)?;
                    Ok(())
                },
                EntryValidationData::Delete {validation_data,.. } => {
                    validation::validate_only_teacher_can_do(validation_data)?;
                    Ok(())
                }
            }
        },
        links:[
            to!(
                Content::entry_type(),
                link_type: SECTION_TO_CONTENT_LINK,
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            )
        ]
    )
}
