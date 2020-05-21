pub struct Ticket {
    ticket_info: TicketInfo,
    signature: String,
}

pub struct TicketInfo {
    source_item_id: String,
    issuer_address: String,
    timestamp: usize,
    buyer_address: String,
}

impl TicketInfo {
    pub fn Get_Signature(&self) -> String {
        return "get signature using issuer_address".to_string();
    }
}

pub fn decode(_token: String) -> Result<Ticket, String> {
    Ok(Ticket {
        source_item_id: "dna_Id".to_string(),
        issuer_address: "asdfasdfsfd".to_string(),
        timestamp: 12,
        ticket_type: TicketType::Free,
        signature: "something".to_string(),
    })
}
