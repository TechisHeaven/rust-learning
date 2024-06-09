use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use std::sync::Mutex;

pub mod deserialize;
pub mod query;
pub mod state_extractor;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn user_id_friend(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

pub async fn server() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/", web::get().to(index))
            .service(
                web::scope("/www").route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(user_id_friend)
            .service(
                web::scope("/user")
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
