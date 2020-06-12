use hdk::ValidationData;

pub fn validate_only_teacher_can_do(validation_data: ValidationData) -> Result<(), String> {
    if !validation_data
        .sources()
        .contains(&crate::helper::get_teacher_address()?)
    {
        return Err(format!("Only the teacher can run this operation"));
    }
    Ok(())
}
