use super::validation;
use crate::payment::paymentitem::entry::PaymentItem;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PaymentInfo {
    owner_address: Address,
    id: u8,
}

impl HolochainEntry for PaymentInfo {
    fn entry_type() -> String {
        String::from("paymentinfo")
    }
}

pub fn paymentinfo_entry_def() -> ValidatingEntryType {
    entry!(
        name: PaymentInfo::entry_type(),
        description:"A category for payment item or items ",
        sharing: Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|validation_data: hdk::EntryValidationData<PaymentInfo>|{
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
                },
        links:[
            to!(
                PaymentItem::entry_type(),
                link_type: "paymentInfo->paymentItem",
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    // ToDo: Implement validation.
                    Ok(())
                }
            )
        ]
    )
}
