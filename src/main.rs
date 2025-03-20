use std::hash;

use actix_cors::Cors;
use actix_web::{http, post, web::Json, App, HttpResponse, HttpServer};
use e_learning_cargo::{
    hash_password::{hash_password::hash_custom_password, verify_password::verify_password},
    models::AppUsers,
};

#[post("/create_app_user")]
pub async fn create_app_user(data: Json<AppUsers>) -> HttpResponse {
    let data_to_create = AppUsers {
        username: data.username.clone().to_uppercase(),
        useremail: data.useremail.clone(),
        userpassword: data.userpassword.clone().to_uppercase(),
    };
    HttpResponse::Ok().body("user created")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let hashed_password = hash_custom_password("password");
    let entered_password = String::from("passworD");
    let is_user = verify_password(hashed_password, entered_password);
    if is_user {
        println!("You are the current user");
    } else {
        println!("You are not the current user");
    }
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:4200")
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(create_app_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
