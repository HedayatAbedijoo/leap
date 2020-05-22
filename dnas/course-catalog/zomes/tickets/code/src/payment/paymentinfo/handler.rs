use super::entry::PaymentInfo;
use hdk::{error::ZomeApiResult, prelude::*};
use holochain_entry_utils::HolochainEntry;

pub fn create(id: u8) -> ZomeApiResult<Address> {
    let new_item = PaymentInfo::new(id, hdk::AGENT_ADDRESS.clone());
    let address = hdk::commit_entry(&new_item.entry())?;
    Ok(address)
}
