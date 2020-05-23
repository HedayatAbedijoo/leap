use hdk::prelude::*;
pub fn get_teacher_address() -> ZomeApiResult<Address> {
    let initial_members_json = hdk::property("teacher_address")?;
    let initial_members: Result<Address, _> =
        serde_json::from_str(&initial_members_json.to_string());

    match initial_members {
        Ok(initial_members_addresses) => Ok(initial_members_addresses),
        Err(_) => Err(ZomeApiError::from(String::from(
            "Could not get the teacher_address for this app",
        ))),
    }
}
