use diesel::prelude::*;

use crate::connection::connection::establish_connection;
use crate::models::AppUsers;
use crate::schema::app_users;

pub fn create_app_user(data: AppUsers) -> Result<AppUsers, diesel::result::Error> {
    let connection = &mut establish_connection();
    diesel::insert_into(app_users::dsl::app_users)
        .values(data)
        .returning(AppUsers::as_returning())
        .get_result(connection)
}
