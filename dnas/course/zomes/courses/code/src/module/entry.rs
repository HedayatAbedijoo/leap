use hdk::holochain_persistence_api::cas::content::Address;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use super::validation;

pub struct Module {
    title: String,
    course_anchor_address: Address,
    timestamp: i32,
}

pub const MODULE_ENTRY_NAME: &str = "module";
pub const MODULE_ENTRY_DESCRIPTION: &str = "Module entry contains single module from some course";

impl HolochainEntry for Module {
    fn entry_type() -> String {
        String::from(MODULE_ENTRY_NAME)
    }
}

pub fn module_entry_def() -> ValidatingEntryType {
    entry!(
        name: MODULE_ENTRY_NAME,
        description: MODULE_ENTRY_DESCRIPTION,
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Module>| {
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
