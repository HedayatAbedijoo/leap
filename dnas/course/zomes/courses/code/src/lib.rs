#![feature(proc_macro_hygiene)]
#![feature(vec_remove_item)]

use hdk::prelude::*;
use hdk_proc_macros::zome;
extern crate holochain_entry_utils;
mod content;
mod course;
mod helper;
mod section;
#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn course_entry_def() -> ValidatingEntryType {
        return crate::course::entry::course_entry_def();
    }

    #[entry_def]
    fn thecourse_anchor_def() -> ValidatingEntryType {
        return crate::course::anchor::the_course_anchor_def();
    }

    #[zome_fn("hc_public")]
    fn create_course(title: String, timestamp: u64) -> ZomeApiResult<Address> {
        crate::course::handlers::create(title, timestamp)
    }

    #[zome_fn("hc_public")]
    fn update_course(
        title: String,
        sections_addresses: Vec<Address>,
        course_address: Address,
    ) -> ZomeApiResult<Address> {
        crate::course::handlers::update(title, sections_addresses, &course_address)
    }

    /**************************** section Entry Definition & Functions */
    #[entry_def]
    fn section_entry_def() -> ValidatingEntryType {
        return crate::section::entry::section_entry_def();
    }
    #[zome_fn("hc_public")]
    fn create_section(
        title: String,
        course_address: Address,
        timestamp: u64,
    ) -> ZomeApiResult<Address> {
        crate::section::handlers::create(title, &course_address, timestamp)
    }
    #[zome_fn("hc_public")]
    fn update_section(title: String, section_address: Address) -> ZomeApiResult<Address> {
        crate::section::handlers::update(title, &section_address)
    }
    #[zome_fn("hc_public")]
    fn delete_section(section_address: Address, course_address: Address) -> ZomeApiResult<Address> {
        crate::section::handlers::delete(section_address, course_address)
    }

    /**************************** Content Entry Definition & Functions */
    #[entry_def]
    fn content_entry_def() -> ValidatingEntryType {
        return crate::content::entry::content_entry_def();
    }

    #[zome_fn("hc_public")]
    fn create_content(
        title: String,
        section_address: Address,
        url: String,
        timestamp: u64,
        description: String,
    ) -> ZomeApiResult<Address> {
        crate::content::handlers::create(section_address, title, url, timestamp, description)
    }

    #[zome_fn("hc_public")]
    fn update_content(
        content_address: Address,
        title: String,
        url: String,
        description: String,
    ) -> ZomeApiResult<Address> {
        crate::content::handlers::update(content_address, title, url, description)
    }

    #[zome_fn("hc_public")]
    fn delete_content(content_address: Address, section_address: Address) -> ZomeApiResult<Address> {
        crate::content::handlers::delete(content_address, section_address)
    }
}
