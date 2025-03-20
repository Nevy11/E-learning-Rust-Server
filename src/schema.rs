// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (username) {
        username -> Varchar,
        useremail -> Varchar,
        userpassword -> Varchar,
    }
}
