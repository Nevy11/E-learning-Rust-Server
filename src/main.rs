use e_learning_cargo::test;
use test::test_file1::hash_password;
fn main() {
    let password = b"Skyworth.95";
    let hashed_password = hash_password(password);
    println!("{}", hashed_password);
    let different_login = b"Skyworth.95";
    let matches = argon2::verify_encoded(&hashed_password, different_login).unwrap();
    println!("does it match? {matches}")
}
