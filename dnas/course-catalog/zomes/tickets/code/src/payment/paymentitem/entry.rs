use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct PaymentItem {
    title: String,
    symbol: String,
    price: usize,
    reciever_handle: String,
    payment_gateway_address: String,
    owner_address: Address,
}

impl HolochainEntry for PaymentItem {
    fn entry_type() -> String {
        String::from("paymentitem")
    }
}
