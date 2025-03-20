use diesel::{pg::Pg, prelude::*};
use serde::Deserialize;

#[derive(Insertable, Selectable, Deserialize, Queryable)]
#[diesel(table_name=crate::schema::app_users)]
#[diesel(check_for_backend(Pg))]
pub struct AppUsers {
    pub username: String,
    pub useremail: String,
    pub userpassword: String,
}
