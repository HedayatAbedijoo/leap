use hdk::prelude::*;
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Ticket {
    pub ticket_info: TicketInfo,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TicketInfo {
    pub source_item_id: String,
    pub issuer_address: String,
    pub timestamp: usize,
    pub buyer_address: String,
}

impl TicketInfo {
    pub fn generate_signature(&self) -> String {
        return "get signature using issuer_address".to_string();
    }

    pub fn new(
        source_item_id: String,
        issuer_address: String,
        timestamp: usize,
        buyer_address: String,
    ) -> Self {
        TicketInfo {
            source_item_id,
            issuer_address,
            timestamp,
            buyer_address,
        }
    }
}

impl Ticket {
    pub fn new(ticket_info: TicketInfo) -> Self {
        Ticket {
            ticket_info: ticket_info.clone(),
            signature: ticket_info.generate_signature(),
        }
    }

    pub fn issue_free_ticket(
        source_item_id: String,
        issuer_address: String,
        timestamp: usize,
    ) -> Self {
        let ticket_info =
            TicketInfo::new(source_item_id, issuer_address, timestamp, "*".to_string());

        return Ticket::new(ticket_info);
    }
}

pub fn decode(token: String) -> Result<Ticket, String> {
    let ticket = Ticket::new(TicketInfo::new(
        "source_item_id".to_string(),
        "issuer_address".to_string(),
        123,
        token,
    ));

    return Ok(ticket);
}
