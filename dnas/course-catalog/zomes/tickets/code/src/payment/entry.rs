use hdk::prelude::*;
pub struct PaymentInfo {
    owner: Address,
    source_address: Address,
}

pub struct PaymentItem {
    title: String,
    symbol: String,
    price: usize,
    reciever_handle: String,
    payment_gateway_address: String,
    owner: Address,
    payment_info: Address,
}
