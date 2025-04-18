use actix_cors::Cors;
use actix_web::{http, post, web::Json, App, HttpResponse, HttpServer, Responder};
use e_learning_cargo::{
    app_users::{
        create_app_user::create_app_user, read_app_user::read_one_app_user_email,
        update_app_user::update_password_of_user,
    },
    email::email::email,
    hash_password::{hash_password::hash_custom_password, verify_password::verify_password},
    models::{
        AppUsers, AppUsersReturn, HashCheck, HashValue, Login, Otp, ReturnHashCheck,
        ReturnHashValue, ReturnLogin, ReturnOtp, ReturnUpdatePassword, UpdatePassword,
    },
    totp_verification::totp_verification_one::generate_totp,
};

#[post("/create_app_user_server")]
pub async fn create_app_user_server(data: Json<AppUsers>) -> HttpResponse {
    let data_to_create = AppUsers {
        username: data.username.clone().to_uppercase(),
        useremail: data.useremail.clone().to_uppercase(),
        userpassword: data.userpassword.clone(),
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

#[post("/hash_value")]
pub async fn hash_value(data: Json<HashValue>) -> impl Responder {
    let value = hash_custom_password(&data.value.clone());
    let return_value = ReturnHashValue {
        hashed_value: value,
    };
    HttpResponse::Ok().json(return_value)
}

#[post("/verify_hash")]
pub async fn verify_hash(data: Json<HashCheck>) -> impl Responder {
    let hashed_value = data.hashed_value.clone();
    let entered_value = data.entered_value.clone();

    match verify_password(hashed_value, entered_value) {
        Ok(verified) => {
            if verified {
                let returned_value = ReturnHashCheck {
                    matches: true,
                    is_success: true,
                    message: String::new(),
                };
                HttpResponse::Ok().json(returned_value)
            } else {
                let returned_value = ReturnHashCheck {
                    matches: false,
                    is_success: true,
                    message: String::new(),
                };
                HttpResponse::Ok().json(returned_value)
            }
        }
        Err(e) => {
            let returned_value = ReturnHashCheck {
                matches: false,
                is_success: false,
                message: e.to_string(),
            };
            HttpResponse::Ok().json(returned_value)
        }
    }
}

#[post("/send_otp")]
pub async fn send_otp(data: Json<Otp>) -> impl Responder {
    let otp = generate_totp();
    let response = email(&data.email_address.clone(), otp.clone());
    match response {
        Ok(_) => {
            let hashed_otp = hash_custom_password(&otp.to_string());
            let returned_value = ReturnOtp {
                email_address: data.email_address.clone(),
                is_success: true,
                hashed_otp: hashed_otp,
                message: String::from(""),
            };
            HttpResponse::Ok().json(returned_value)
        }
        Err(error) => {
            let returned_value = ReturnOtp {
                email_address: data.email_address.clone(),
                is_success: false,
                hashed_otp: String::new(),
                message: error.to_string(),
            };
            HttpResponse::Ok().json(returned_value)
        }
    }
}

#[post("/login")]
pub async fn login(data: Json<Login>) -> impl Responder {
    let email_of_user = data.email_address.clone();
    let user_data = read_one_app_user_email(email_of_user);
    match user_data {
        Ok(fetched_data) => {
            let entered_password = data.password.clone();
            println!("Entered_password\n: {}", entered_password);
            let stored_password = fetched_data.userpassword.clone();
            println!("Stored_password\n: {}", stored_password);
            let correct_result = verify_password(stored_password, entered_password);
            match correct_result {
                Ok(is_correct) => {
                    if is_correct {
                        let return_data = ReturnLogin {
                            is_correct: true,
                            is_success: true,
                            message: String::new(),
                        };
                        HttpResponse::Ok().json(return_data)
                    } else {
                        let return_data = ReturnLogin {
                            is_correct: false,
                            is_success: true,
                            message: String::new(),
                        };
                        HttpResponse::Ok().json(return_data)
                    }
                }
                Err(_) => {
                    let return_data = ReturnLogin {
                        is_correct: false,
                        is_success: false,
                        message: String::new(),
                    };
                    HttpResponse::Ok().json(return_data)
                }
            }
        }
        Err(e) => {
            let return_data = ReturnLogin {
                is_correct: false,
                is_success: false,
                message: e.to_string(),
            };
            HttpResponse::Ok().json(return_data)
        }
    }
}

#[post("/update_user_password")]
pub async fn update_user_password(data: Json<UpdatePassword>) -> impl Responder {
    let updated_user = update_password_of_user(
        data.user_email.clone().to_uppercase(),
        data.new_password.clone(),
    );
    match updated_user {
        Ok(_) => {
            let returned_data = ReturnUpdatePassword {
                user_email: data.user_email.clone(),
                is_success: true,
                message: String::new(),
            };
            HttpResponse::Ok().json(returned_data)
        }
        Err(err) => {
            let returned_data = ReturnUpdatePassword {
                user_email: data.user_email.clone(),
                is_success: false,
                message: err.to_string(),
            };
            HttpResponse::Ok().json(returned_data)
        }
    }
}
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("done");
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
            .service(hash_value)
            .service(send_otp)
            .service(verify_hash)
            .service(login)
            .service(update_user_password)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
