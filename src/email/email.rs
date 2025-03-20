use dotenvy::dotenv;
use lettre::{
    transport::smtp::{authentication::Credentials, response::Response},
    Message, SmtpTransport, Transport,
};
use std::env;

pub fn email(to: &str, otp: String) -> Result<Response, lettre::transport::smtp::Error> {
    dotenv().ok();
    let username = env::var("EMAILUSERNAME").unwrap();
    let password = env::var("EMAILPASSWORD").unwrap();
    let email = Message::builder()
        .from(username.parse().unwrap())
        .to(to.parse().unwrap())
        .subject("OTP code")
        .body(otp)
        .unwrap();
    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    let response: Result<Response, lettre::transport::smtp::Error> = mailer.send(&email);
    response
}
