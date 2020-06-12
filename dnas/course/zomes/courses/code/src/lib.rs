#![feature(proc_macro_hygiene)]
#![feature(vec_remove_item)]

use hdk::prelude::*;
use hdk_proc_macros::zome;
extern crate holochain_entry_utils;
mod course;
mod helper;
mod module;
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
        modules_addresses: Vec<Address>,
        course_address: Address,
    ) -> ZomeApiResult<Address> {
        crate::course::handlers::update(title, modules_addresses, &course_address)
    }

    /**************************** Module Entry Definition & Functions */
    #[entry_def]
    fn module_entry_def() -> ValidatingEntryType {
        return crate::module::entry::module_entry_def();
    }
    #[zome_fn("hc_public")]
    fn create_module(
        title: String,
        course_address: Address,
        timestamp: u64,
    ) -> ZomeApiResult<Address> {
        crate::module::handlers::create(title, &course_address, timestamp)
    }
    #[zome_fn("hc_public")]
    fn update_module(title: String, module_address: Address) -> ZomeApiResult<Address> {
        crate::module::handlers::update(title, &module_address)
    }
    #[zome_fn("hc_public")]
    fn delete_module(module_address: Address, course_address: Address) -> ZomeApiResult<Address> {
        crate::module::handlers::delete(module_address, course_address)
    }
}
