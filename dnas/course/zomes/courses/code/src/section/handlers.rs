use super::entry::{Section, SECTION_TO_CONTENT_LINK};
use crate::course;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

pub fn create(title: String, course_address: &Address, timestamp: u64) -> ZomeApiResult<Address> {
    //add new Section to DHT
    let new_section = Section::new(title, timestamp);
    let new_section_address = hdk::commit_entry(&new_section.entry())?;

    course::handlers::add_new_section(&course_address, &new_section_address)?;
    Ok(new_section_address)
}

pub fn update(title: String, section_address: &Address) -> ZomeApiResult<Address> {
    let mut section: Section = hdk::utils::get_as_type(section_address.clone())?;
    section.update(title);
    hdk::update_entry(section.entry(), section_address)
}

pub fn delete(section_address: Address, course_address: Address) -> ZomeApiResult<Address> {
    let result = hdk::remove_entry(&section_address)?;
    course::handlers::remove_section(&course_address, &section_address)?;
    Ok(result)
}

pub fn link_new_content_to_section(
    section_address: Address,
    content_address: Address,
) -> ZomeApiResult<Address> {
    hdk::link_entries(
        &section_address,
        &content_address,
        SECTION_TO_CONTENT_LINK,
        "",
    )
}

pub fn remove_content_link_from_section(
    section_address: Address,
    content_address: Address,
) -> ZomeApiResult<()> {
    hdk::remove_link(
        &section_address,
        &content_address,
        SECTION_TO_CONTENT_LINK,
        "",
    )
}
