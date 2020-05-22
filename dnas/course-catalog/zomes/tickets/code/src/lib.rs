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
        return crate::payment::paymentinfo::entry::paymentinfo_entry_def();
    }

    #[entry_def]
    fn paymentitem_entry_def() -> ValidatingEntryType {
        return crate::payment::paymentitem::entry::paymentitem_entry_def();
    }

    #[zome_fn("hc_public")]
    pub fn create_payment_info(id: u8) -> ZomeApiResult<Address> {
        return crate::payment::paymentinfo::handler::create(id);
    }

    #[zome_fn("hc_public")]
    pub fn create_payment_item(
        title: String,
        symbol: String,
        price: usize,
        reciever_handle: String,
        payment_gateway_address: String,
        payment_info_id: u8,
    ) -> ZomeApiResult<Address> {
        return crate::payment::paymentitem::handler::create(
            title,
            symbol,
            price,
            reciever_handle,
            payment_gateway_address,
            payment_info_id,
        );
    }
}
