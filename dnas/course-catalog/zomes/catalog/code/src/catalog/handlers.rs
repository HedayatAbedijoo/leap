use super::{
    anchors,
    entry::{Catalog, PaymentInfo, LINK_AGENT_ADDRESS_TO_CATALOG},
    ticket::Ticket,
};
use anchors::{AllCatalogAnchor, AnchorTrait};
use hdk::AGENT_ADDRESS;
use hdk::{error::ZomeApiResult, prelude::*};
use holochain_entry_utils::HolochainEntry;

pub fn create_free_catalog(
    course_title: String,
    course_id: String,
    publish_time: usize,
) -> ZomeApiResult<Address> {
    let free_ticket = Ticket::issue_free_ticket(
        course_id.clone(),
        AGENT_ADDRESS.clone(),
        publish_time.clone(),
    );

    let new_item = Catalog::new(
        course_title,
        course_id,
        AGENT_ADDRESS.clone(),
        publish_time,
        PaymentInfo::Free {
            ticket: free_ticket,
        },
    );

    let catalog_address = hdk::commit_entry(&new_item.entry())?;
    hdk::link_entries(
        &AGENT_ADDRESS,
        &catalog_address,
        LINK_AGENT_ADDRESS_TO_CATALOG,
        "",
    )?;

    Ok(catalog_address)
}

pub fn create_catalog_with_payment_info(
    course_title: String,
    course_id: String,
    publish_time: usize,
    payment_info_address: Address,
) -> ZomeApiResult<Address> {
    let new_item = Catalog::new(
        course_title,
        course_id,
        AGENT_ADDRESS.clone(),
        publish_time,
        PaymentInfo::PyamentOptions {
            payment_info_address: payment_info_address,
        },
    );

    let catalog_address = hdk::commit_entry(&new_item.entry())?;
    hdk::link_entries(
        &AGENT_ADDRESS,
        &catalog_address,
        LINK_AGENT_ADDRESS_TO_CATALOG,
        "",
    )?;

    let all_catalog = AllCatalogAnchor {};
    hdk::link_entries(
        &all_catalog.address()?,
        &catalog_address,
        AllCatalogAnchor::link_type(),
        "".to_string(),
    )?;
    Ok(catalog_address)
}
