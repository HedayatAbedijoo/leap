#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_entry_utils;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk_proc_macros::zome;

pub mod settings;
pub mod ticket;

#[zome]
mod my_zome {
    use hdk::EntryValidationData;
    use ticket::Ticket;

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        match validation_data {
            EntryValidationData::Create {
                validation_data, ..
            } => {
                let agent_address = validation_data.package.chain_header.provenances()[0].source();

                let nick_name = "dummy_data".to_string();

                let ticket: Ticket = ticket::decode(nick_name)?;
                let prop_course_id = settings::get_dna_id()?;
                let prop_teacher_addr = settings::get_teacher_address()?;

                if agent_address == prop_teacher_addr {
                    return Ok(());
                }
                if ticket.ticket_info.source_item_id != prop_course_id {
                    return Err(String::from(
                        "Validation Error:  ticket is not issed for this DNA",
                    ));
                }

                if ticket.ticket_info.buyer_address == "*".to_string() // free course
                    && ticket.signature != ticket.ticket_info.generate_signature()
                {
                    return Err(String::from("Validation Error:  ticket is not valid"));
                }

                if ticket.ticket_info.buyer_address != agent_address.to_string() {
                    return Err(String::from(
                        "Validation Error:  ticket is not issued for you",
                    ));
                }
                if ticket.ticket_info.buyer_address == agent_address.to_string()
                    && ticket.signature != ticket.ticket_info.generate_signature()
                {
                    return Err(String::from("Validation Error:  ticket is not valid"));
                }
            }
            _ => {
                return Err(String::from("Validation Error:  validating the agent"));
            }
        }

        Ok(())
    }
}
