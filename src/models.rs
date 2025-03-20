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
