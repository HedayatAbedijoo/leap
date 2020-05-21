#![feature(proc_macro_hygiene)]
#![allow(dead_code)]

use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/0.0.47-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry
mod settings;
mod ticket;
#[zome]
mod my_zome {

    use hdk::EntryValidationData;

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        let nick_name = "nick_name_with_data".to_string(); //hdk::prelude::AgentId::decoded_key()?;
        let _ticket = ticket::decode(nick_name)?;
        let _prop_teacher_address = crate::settings::get_teacher_address()?;
        let _prop_dna_id = crate::settings::get_dna_id()?;
        let _agent_address = validation_data.package.chain_header.provenances()[0].source();

        /// if   _prop_teacher_address == _agent_address   return OK(())
        /// else check ticket
        // if ticket.ticket_info.buyer== *  =>
        ///   then if thicket.ticket_info.Get_Signature() == ticket.signature()  /// it freee   reutrn OK(())  else/// else
        /// else then    if ticket.ticket_info.buyer_address == _agent_address   and  validate_Siganture /// ture
        match validation_data {
            EntryValidationData::Create {
                validation_data, ..
            } => {
                let agent_address = validation_data.package.chain_header.provenances()[0].source();
                hdk::debug(format!(
                    "hedayat: agent address in validation_data {:?}",
                    agent_address.clone()
                ))?;

                Ok(())
                //  Ok(())
                // match members::is_valid_member(&agent_address)? {
                //     true => Ok(()),
                //     false => Err(format!("Validation Error: Agent {} is not valid since it does not comply with social triangulation conditions", agent_address))
                // }
            }
            _ => Err(String::from("Validation Error:  validating the agent")),
        } // TODO:
          // if ()

        Ok(())
    }
}
