use totp_rs::{Secret, TOTP};

pub fn generate_totp(secret_key: String) -> String {
    let secret = Secret::Encoded(secret_key);
    let totp = TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes().unwrap(),
    )
    .unwrap();
    totp.generate_current().unwrap()
}
