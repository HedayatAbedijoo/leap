pub struct Ticket {
    ticket_info: TicketInfo,
    signature: String,
}

pub struct TicketInfo {
    source_item_id: String,
    issuer_address: String,
    timestamp: usize,
    ticket_type: TicketType,
    buyer_address: String,
}

impl TicketInfo {
    pub fn Get_Signature(&self) -> String {
        return "get signature using issuer_address".to_string();
    }
}

pub fn serialize(ticket: Ticket) -> String {}
