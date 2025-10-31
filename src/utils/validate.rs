use validator::ValidationError;

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^1[3-9]\d{9}$").unwrap();
    if re.is_match(phone) {
        return Ok(());
    }
    Err(ValidationError::new("3"))
}
