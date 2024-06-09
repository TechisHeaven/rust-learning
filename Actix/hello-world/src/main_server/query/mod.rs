use actix_web::{error, get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    username: String,
}

// this handler gets called if the query deserializes into `Info` successfully
// otherwise a 400 Bad Request error response is returned
#[get("/")]
async fn index(info: web::Query<Info>) -> String {
    if info.username.len() > 0 {
        format!("Welcome {}!", info.username)
    } else {
        format!("No Welcome without username")
    }
}

/// deserialize `Info` from request's body
async fn submit(info: web::Json<Info>) -> impl Responder {
    // Ok(format!("Welcome {}!", info.username))
    format!("Welcome {}", info.username)
}

pub async fn query_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new().service(
            web::resource("/submit")
                // change json extractor configuration
                .app_data(json_config)
                .route(web::post().to(submit)),
        )
        // ! .service(submit)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
