use super::validation;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Content {
    pub title: String,
    pub url: String,
    pub timestamp: u64,
    pub description: String,
}

impl Content {
    pub fn new(title: String, url: String, timestamp: u64, description: String) -> Self {
        Content {
            title,
            url,
            timestamp,
            description,
        }
    }

    pub fn update(&mut self, title: String, url: String, description: String) {
        self.title = title;
        self.url = url;
        self.description = description;
        self.timestamp += 1;
    }
}

impl HolochainEntry for Content {
    fn entry_type() -> String {
        String::from("content")
    }
}

pub fn content_entry_def() -> ValidatingEntryType {
    entry!(
        name: Content::entry_type(),
        description: "Content entry stores single piece of content from a course module",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |validation_data: hdk::EntryValidationData<Content>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    validation::create(entry,validation_data)?;
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry,old_entry_header, validation_data } => {
                    validation::modify(new_entry,old_entry,old_entry_header,validation_data)?;
                    Ok(())
                },
                EntryValidationData::Delete {old_entry,old_entry_header, validation_data } => {
                    validation::delete(old_entry, old_entry_header, validation_data)?;
                    Ok(())
                }
            }
        },
        links: [ ]
    )
}
