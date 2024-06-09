use actix_web::{get, web, HttpRequest, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id {}!",
        info.friend, info.user_id
    ))
}

#[get("/users2/{user_id}/{name}")]
async fn second_index(req: HttpRequest) -> Result<String> {
    let name: String = req.match_info().get("name").unwrap().parse().unwrap();

    Ok(format!("Hello {name}"))
}

pub async fn deserialize_server() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index).service(second_index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
