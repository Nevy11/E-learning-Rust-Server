pub fn verify_password(
    hashed_password: String,
    entered_password: String,
) -> Result<bool, argon2::Error> {
    let entered_password = entered_password.as_bytes();
    let matches = argon2::verify_encoded(&hashed_password, entered_password);
    matches
}
