use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};

#[allow(unused_variables, dead_code)]
pub fn hash_password(password: &str) {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
}
