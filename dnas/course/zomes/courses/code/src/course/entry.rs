use super::validation;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    pub title: String,
    pub sections: Vec<Address>,
    // anchor_address: Address, // implicit link to CourseAnchor to be able to quickly retrieve the corresponding anchor
    timestamp: u64,
}

impl HolochainEntry for Course {
    fn entry_type() -> String {
        String::from("course")
    }
}

impl Course {
    pub fn new(title: String, timestamp: u64) -> Self {
        Course {
            title: title,
            timestamp: timestamp,
            //anchor_address: anchor_address,
            sections: Vec::default(),
        }
    }

    pub fn update_from(old_entry: Course, title: String, sections: Vec<Address>) -> Self {
        Course {
            title: title,
            timestamp: old_entry.timestamp + 1,
            sections: sections,
        }
    }
}

pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: Course::entry_type(),
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
