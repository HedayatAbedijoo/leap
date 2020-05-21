use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;




#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Catalog {
    pub course_title: String,
    pub course_id: String,
    pub teacher_id: Address,
    pub publish_time: usize,
    pub price: PaymentInfo,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum PaymentInfo {
    Free,
    PyamentOptions { payment_info_address: Address },
}

impl HolochainEntry for Catalog {
    fn entry_type() -> String {
        String::from("attestation")
    }
}



/// entry definition


/// 