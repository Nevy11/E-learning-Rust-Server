// @generated automatically by Diesel CLI.

diesel::table! {
    app_users (username) {
        username -> Varchar,
        useremail -> Nullable<Varchar>,
        userpassword -> Varchar,
    }
}
