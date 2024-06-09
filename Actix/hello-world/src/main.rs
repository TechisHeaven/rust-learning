// use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};

// #[get("/")]
// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello World!")
// }

// #[get("/{username}/{id}/index.html")] // <- define path parameters
// async fn index2(info: web::Path<(String, u32)>) -> Result<String> {
//     let info = info.into_inner();
//     Ok(format!("Welcome {}! id: {}", info.0, info.1))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().service(index2).service(index))
//         .bind(("127.0.0.1", 8080))?
//         .run()
//         .await
// }

//! second example ---------------------------------

// use actix_web::{get, web, App, HttpServer};

// // This struct represents state
// struct AppState {
//     app_name: String,
// }

// #[get("/")]
// async fn index(data: web::Data<AppState>) -> String {
//     let app_name = &data.app_name; // <- get app_name
//     format!("Hello {app_name}!") // <- response with app_name
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .app_data(web::Data::new(AppState {
//                 app_name: String::from("Actix Web"),
//             }))
//             .service(index)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

//! third example

pub mod main_server;

use actix_web;
// use main_server::deserialize::deserialize_server;
use main_server::query::query_server;
use main_server::state_extractor::state_extractor;
// use main_server::server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    // return server().await;
    // return deserialize_server().await;
    // return query_server().await;
    return state_extractor().await;
}
