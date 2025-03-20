use std::env;

use argon2::Config;
use dotenvy::dotenv;

pub fn hash_custom_password(password: &str) -> String {
    let password = password.as_bytes();
    dotenv().ok();
    let salt = env::var("SALT").expect("SALT NOT FOUND\nPLEASE ADD SALT IN THE .env variable");
    println!("{salt}");
    let salt = salt.as_bytes();
    let config = Config::default();
    let hash = argon2::hash_encoded(password, salt, &config).unwrap();
    println!("{}", hash);
    hash
}
