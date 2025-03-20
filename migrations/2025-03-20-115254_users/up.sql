-- Your SQL goes here
CREATE TABLE app_users(
    username VARCHAR PRIMARY KEY,
    useremail VARCHAR UNIQUE,
    userpassword VARCHAR NOT NULL
);