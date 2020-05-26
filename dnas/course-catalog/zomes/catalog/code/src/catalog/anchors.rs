use super::entry::Catalog;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct AllCatalogAnchor {}

impl AnchorTrait for AllCatalogAnchor {
    fn entry_type() -> String {
        String::from("SELECT_ALL_CATALOG")
    }
    fn link_to() -> String {
        Catalog::entry_type()
    }
    fn link_type() -> String {
        "SELECT_*_From_Catalog".to_string()
    }
}

// Generic Trait for Anchor patten. We can put it in crate.io later.
pub trait AnchorTrait: TryFrom<JsonString> + Into<JsonString> + Clone {
    fn entry_type() -> String;
    fn link_to() -> String;
    fn link_type() -> String;
    fn entry(self) -> Entry {
        Entry::App(Self::entry_type().into(), self.into())
    }

    fn address(&self) -> ZomeApiResult<Address> {
        let addrss = hdk::commit_entry(&self.clone().entry())?;
        return Ok(addrss);

        // TODO: Investigation: Which one is better
        // let address = hdk::entry_address(&self.clone().entry())?;
        // if let Ok(None) = hdk::get_entry(&address) {
        //     return Ok(address);
        // } else {
        //     let addrss = hdk::commit_entry(&self.clone().entry())?;
        //     return Ok(addrss);
        // }
    }
}

pub fn anchor_select_all_catalog_def() -> ValidatingEntryType {
    entry!(
        name: AllCatalogAnchor::entry_type(),
        description:"Anchor to all Courses",
        sharing: Sharing::Public,
        validation_package:||{
            hdk::ValidationPackageDefinition::Entry
        },
        validation:|_validation_data: hdk::EntryValidationData<AllCatalogAnchor>|{
            Ok(())
        },
        links:[
            to!(
                AllCatalogAnchor::link_to(),
                link_type: AllCatalogAnchor::link_type(),
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            )
        ]
    )
}
