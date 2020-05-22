#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

mod payment;

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
    fn paymentinfo_entry_def() -> ValidatingEntryType {
        return payment::paymentinfo::entry::paymentinfo_entry_def();
    }
}
