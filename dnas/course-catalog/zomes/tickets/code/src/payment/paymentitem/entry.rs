use super::validation;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PaymentItem {
    title: String,
    symbol: String,
    price: usize,
    reciever_handle: String,
    payment_gateway_address: String,
    pub owner_address: Address,
}

impl PaymentItem {
    pub fn new(
        title: String,
        symbol: String,
        price: usize,
        reciever_handle: String,
        payment_gateway_address: String,
    ) -> Self {
        PaymentItem {
            title,
            symbol,
            price,
            reciever_handle,
            payment_gateway_address,
            owner_address: hdk::AGENT_ADDRESS.clone(),
        }
    }
}

impl HolochainEntry for PaymentItem {
    fn entry_type() -> String {
        String::from("paymentitem")
    }
}

pub fn paymentitem_entry_def() -> ValidatingEntryType {
    entry!(
        name: PaymentItem::entry_type(),
        description:"A payment item for paying purpose.",
        sharing: Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|validation_data: hdk::EntryValidationData<PaymentItem>|{
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                   validation::create(entry,validation_data)?;
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry,old_entry_header, validation_data } => {
                    validation::modify( new_entry,old_entry,old_entry_header,validation_data,)?;
                    Ok(())
                },
                EntryValidationData::Delete {old_entry,old_entry_header, validation_data } => {
                 validation::delete(old_entry, old_entry_header, validation_data)?;
                    Ok(())
                }
            }
                }
    )
}
