use std::env;

use dotenvy::dotenv;
use totp_rs::{Secret, TOTP};

pub fn generate_totp() -> String {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY_OTP").unwrap();
    let secret = Secret::Encoded(secret_key);
    let totp = TOTP::new(
        totp_rs::Algorithm::SHA1,
        6,
        1,
        30,
        secret.to_bytes().unwrap(),
    )
    .unwrap();
    println!("{}", totp.generate_current().unwrap());
    totp.generate_current().unwrap()
}
