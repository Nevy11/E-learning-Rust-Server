use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Selectable, Deserialize, Queryable, Clone)]
#[diesel(table_name=crate::schema::app_users)]
#[diesel(check_for_backend(Pg))]
pub struct AppUsers {
    pub username: String,
    pub useremail: String,
    pub userpassword: String,
}

#[derive(Serialize)]
pub struct AppUsersReturn {
    pub username: String,
    pub useremail: String,
    pub is_successful: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct HashValue {
    pub value: String,
}

#[derive(Serialize)]
pub struct ReturnHashValue {
    pub hashed_value: String,
}

#[derive(Deserialize)]
pub struct Otp {
    pub email_address: String,
}

#[derive(Serialize)]
pub struct ReturnOtp {
    pub email_address: String,
    pub hashed_otp: String,
    pub is_success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct HashCheck {
    pub hashed_value: String,
    pub entered_value: String,
}

#[derive(Serialize)]
pub struct ReturnHashCheck {
    pub matches: bool,
}

#[derive(Deserialize)]
pub struct Login {
    pub email_address: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ReturnLogin {
    pub is_correct: bool,
    pub is_success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub user_email: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct ReturnUpdatePassword {
    pub user_email: String,
    pub is_success: bool,
    pub message: String,
}
