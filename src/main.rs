use actix_cors::Cors;
use actix_web::{http, App, HttpServer};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("hello world");
    HttpServer::new(|| {
        App::new().wrap(
            Cors::default()
                .allowed_origin("http://localhost:4200")
                .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::CONTENT_TYPE,
                ])
                .max_age(3600),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
