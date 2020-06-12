use super::validation;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    title: String,
    modules: Vec<Address>,
    // anchor_address: Address, // implicit link to CourseAnchor to be able to quickly retrieve the corresponding anchor
    timestamp: u64,
}

pub const COURSE_ENTRY_NAME: &str = "course";

impl HolochainEntry for Course {
    fn entry_type() -> String {
        String::from(COURSE_ENTRY_NAME)
    }
}

impl Course {
    pub fn new(title: String, timestamp: u64) -> Self {
        Course {
            title: title,
            timestamp: timestamp,
            //anchor_address: anchor_address,
            modules: Vec::default(),
        }
    }

    pub fn update_from(old_entry: Course, title: String, modules: Vec<Address>) -> Self {
        Course {
            title: title,
            timestamp: old_entry.timestamp + 1,
            modules: modules,
        }
    }
}

pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: COURSE_ENTRY_NAME,
        description: "Course entry contains all course data and is updated on every course change",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data{
                EntryValidationData::Create { validation_data,.. } => {
                    validation::create(validation_data)?;
                     Ok(())
                 },
                 EntryValidationData::Modify { new_entry, old_entry,old_entry_header, validation_data } => {
                     validation::modify( new_entry,old_entry,old_entry_header,validation_data,)?;
                     Ok(())
                 },
                 EntryValidationData::Delete {.. } => {
                    return Err(String::from("Cannot delete the course"));
                 }
            }
        }
    )
}
