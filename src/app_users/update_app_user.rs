use crate::{
    connection::connection::establish_connection,
    models::AppUsers,
    schema::app_users::{self, useremail, username, userpassword},
};
use diesel::prelude::*;

/// Changes the password of the user from the new to the old one
/// The password to be passed here should be hashed
pub fn update_password_of_user(
    email_of_user: String,
    new_password: String,
) -> Result<AppUsers, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::update(app_users::dsl::app_users)
        .filter(useremail.eq(email_of_user.to_uppercase()))
        .set(userpassword.eq(new_password))
        .returning(AppUsers::as_returning())
        .get_result(connection)
}

/// Changes the username of the user of that particular account.
pub fn update_username_of_user(
    email_of_user: String,
    new_username: String,
) -> Result<AppUsers, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::update(app_users::dsl::app_users)
        .filter(useremail.eq(email_of_user.to_uppercase()))
        .set(username.eq(new_username.to_uppercase()))
        .returning(AppUsers::as_returning())
        .get_result(connection)
}
