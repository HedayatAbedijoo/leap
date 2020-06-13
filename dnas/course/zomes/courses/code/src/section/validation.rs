use hdk::ValidationData;

pub fn validate_only_teacher_can_do(validation_data: ValidationData) -> Result<(), String> {
    crate::helper::validate_only_teacher_can_do(validation_data.sources(), "run this operation")
}
