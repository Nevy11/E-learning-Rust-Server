use argon2::Config;

// #[allow(unused_variables, dead_code)]
pub fn hash_password(password: &[u8]) -> String {
    // let password = b"Skyworth.95";
    let salt = b"random_salt";
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    hash
}

// pub fn verify_password() -> boolean {

// }
