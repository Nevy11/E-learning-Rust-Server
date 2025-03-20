use actix_cors::Cors;
use actix_web::{http, post, web::Json, App, HttpResponse, HttpServer, Responder};
use e_learning_cargo::{
    app_users::create_app_user::create_app_user,
    email::email::email,
    hash_password::{hash_password::hash_custom_password, verify_password::verify_password},
    models::{
        AppUsers, AppUsersReturn, HashCheck, HashValue, Otp, ReturnHashCheck, ReturnHashValue,
        ReturnOtp,
    },
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
    let verified = verify_password(hashed_value, entered_value);
    if verified {
        let returned_value = ReturnHashCheck { matches: true };
        HttpResponse::Ok().json(returned_value)
    } else {
        let returned_value = ReturnHashCheck { matches: false };
        HttpResponse::Ok().json(returned_value)
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
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // let generated_otp = generate_totp("OBWGC2LOFVZXI4TJNZTS243FMNZGK5BNGEZDG".to_string());
    // let hashed_otp = hash_custom_password(&generated_otp);
    // println!("{}", generated_otp);
    // let matches = verify_password(hashed_otp, generated_otp);
    // if matches {
    //     println!("login successfully");
    // } else {
    //     println!("Wrong otp");
    // }
    println!("done");
    // email("smongare2004@gmail.com");
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
