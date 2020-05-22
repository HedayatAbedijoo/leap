use super::entry::PaymentItem;
use crate::payment::paymentinfo::entry::{PaymentInfo, PAYMENT_INFO_TO_PAYMENT_ITEM_LINK};
use hdk::{error::ZomeApiResult, prelude::*};
use holochain_entry_utils::HolochainEntry;

pub fn create(
    title: String,
    symbol: String,
    price: usize,
    reciever_handle: String,
    payment_gateway_address: String,
    payment_info_id: u8,
) -> ZomeApiResult<Address> {
    let new_item = PaymentItem::new(
        title,
        symbol,
        price,
        reciever_handle,
        payment_gateway_address,
    );

    let payment_info = PaymentInfo::new(payment_info_id, hdk::AGENT_ADDRESS.clone());
    if payment_info.is_exist_in_dht() == false {
        // To do, create here
        return Err(ZomeApiError::from(String::from(
            "Payment info is not existed. Create Payment info first",
        )));
    }
    let address = hdk::commit_entry(&new_item.entry())?;

    //create link payment_info->Payment_Items
    hdk::link_entries(
        &payment_info.address()?,
        &address,
        PAYMENT_INFO_TO_PAYMENT_ITEM_LINK,
        "",
    )?;

    Ok(address)
}
