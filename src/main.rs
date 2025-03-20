use actix_cors::Cors;
use actix_web::{http, post, web::Json, App, HttpResponse, HttpServer};
use e_learning_cargo::{
    app_users::create_app_user::create_app_user,
    hash_password::{hash_password::hash_custom_password, verify_password::verify_password},
    models::{AppUsers, AppUsersReturn},
    totp_verification::totp_verification_one::generate_totp,
};

#[post("/create_app_user_server")]
pub async fn create_app_user_server(data: Json<AppUsers>) -> HttpResponse {
    let data_to_create = AppUsers {
        username: data.username.clone(),
        useremail: data.useremail.clone(),
        userpassword: hash_custom_password(&data.userpassword.clone()),
    };
    let created_data: Result<AppUsers, diesel::result::Error> = create_app_user(data_to_create);
    match created_data {
        Ok(data) => {
            let return_data = AppUsersReturn {
                username: data.username.clone(),
                useremail: data.useremail.clone(),
                is_successful: true,
                message: String::new(),
            };
            HttpResponse::Ok().json(return_data)
        }
        Err(error) => {
            let return_data = AppUsersReturn {
                username: data.username.clone(),
                useremail: data.useremail.clone(),
                is_successful: false,
                message: error.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let generated_otp = generate_totp("OBWGC2LOFVZXI4TJNZTS243FMNZGK5BNGEZDG".to_string());
    let hashed_otp = hash_custom_password(&generated_otp);
    println!("{}", generated_otp);
    let matches = verify_password(hashed_otp, generated_otp);
    if matches {
        println!("login successfully");
    } else {
        println!("Wrogn otp");
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
            .service(create_app_user_server)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
