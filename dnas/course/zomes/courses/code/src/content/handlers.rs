use super::entry::Content;
use crate::section;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

pub fn create(
    section_address: Address,
    title: String,
    url: String,
    timestamp: u64,
    description: String,
) -> ZomeApiResult<Address> {
    let new_content = Content::new(title, url, timestamp, description);

    let new_content_address = hdk::commit_entry(&new_content.entry())?;
    section::handlers::link_new_content_to_section(section_address, new_content_address.clone())?;
    Ok(new_content_address)
}

pub fn update(
    content_address: Address,
    title: String,
    url: String,
    description: String,
) -> ZomeApiResult<Address> {
    let mut content: Content = hdk::utils::get_as_type(content_address.clone())?;
    content.update(title, url, description);
    hdk::update_entry(content.entry(), &content_address)
}

pub fn delete(content_address: Address, section_address: Address) -> ZomeApiResult<Address> {
    section::handlers::remove_content_link_from_section(section_address, content_address.clone())?;
    hdk::remove_entry(&content_address)
}
