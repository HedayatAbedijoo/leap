#![feature(proc_macro_hygiene)]
// #[macro_use]
// extern crate holochain_entry_utils;

use hdk::prelude::*;
use hdk_proc_macros::zome;
mod catalog;
#[zome]
mod catalog_zome {

    // use crate::catalog::entry::Catalog;

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn catalog_entry_def() -> ValidatingEntryType {
        return crate::catalog::entry::catalog_entry_def();
    }

    // This is anchor will be used for: SELECT * FROM CATALOGS
    #[entry_def]
    fn all_catalog_anchor_def() -> ValidatingEntryType {
        return crate::catalog::anchors::anchor_select_all_catalog_def();
    }

    // Create a Catalog for Course in Index for Free
    #[zome_fn("hc_public")]
    fn create_free_catalog(
        course_title: String,
        course_id: String,
        publish_time: usize,
    ) -> ZomeApiResult<Address> {
        return crate::catalog::handlers::create_free_catalog(
            course_title,
            course_id,
            publish_time,
        );
    }

    // Create a Catalog for Course in Index for Free
    #[zome_fn("hc_public")]
    fn create_payable_catalog(
        course_title: String,
        course_id: String,
        publish_time: usize,
        payment_info_address: Address,
    ) -> ZomeApiResult<Address> {
        return crate::catalog::handlers::create_catalog_with_payment_info(
            course_title,
            course_id,
            publish_time,
            payment_info_address,
        );
    }
}
