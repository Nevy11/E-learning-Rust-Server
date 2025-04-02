use crate::{
    connection::connection::establish_connection,
    models::AppUsers,
    schema::app_users::{self, useremail, username},
};
use diesel::prelude::*;

/// read one app user's data based on Email
pub fn read_one_app_user_email(email_of_user: String) -> Result<AppUsers, diesel::result::Error> {
    let connection = &mut establish_connection();
    app_users::dsl::app_users
        .filter(useremail.eq(email_of_user.to_uppercase()))
        .select(AppUsers::as_returning())
        .get_result(connection)
}

/// Read one app user's data based on username
pub fn read_one_app_user_username(
    username_of_user: String,
) -> Result<AppUsers, diesel::result::Error> {
    let connection = &mut establish_connection();
    app_users::dsl::app_users
        .filter(username.eq(username_of_user.to_uppercase()))
        .select(AppUsers::as_returning())
        .get_result(connection)
}
