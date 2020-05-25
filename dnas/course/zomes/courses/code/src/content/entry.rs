use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use super::validation;

pub struct Content {
    title: String,
    url: String,
    timestamp: i32,
    description: String,
    module_address: Address,
}

pub const CONTENT_ENTRY_NAME: &str = "content";
pub const CONTENT_ENTRY_DESCRIPTION: &str = "Content entry stores single piece of content from a course module";

impl HolochainEntry for Content {
    fn entry_type() -> String {
        String::from(CONTENT_ENTRY_NAME)
    }
}

pub fn content_entry_def() -> ValidatingEntryType {
    entry!(
        name: CONTENT_ENTRY_NAME,
        description: CONTENT_ENTRY_DESCRIPTION,
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Content>| {
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
        links: [ ]
    )
}
