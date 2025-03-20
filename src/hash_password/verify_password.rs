pub fn verify_password(hashed_password: String, entered_password: String) -> bool {
    let entered_password = entered_password.as_bytes();
    let matches = argon2::verify_encoded(&hashed_password, entered_password);
    matches.unwrap()
}
