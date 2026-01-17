mod settings;
mod models;
mod euroleague;

use settings::Settings;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::greeting::Greeting;
use euroleague::euroleague::EuroleagueClient;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Greeting {
        message: "Hello, Rust!".to_string(),
    })
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/games/{season}/{code}")]
async fn get_game(
    client: web::Data<EuroleagueClient>,
    path: web::Path<(String, u32)>,
) -> impl Responder {
    let (season, code) = path.into_inner();
    match client.get_game(&season, code).await {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(e) => {
            eprintln!("Error fetching game: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let settings = Settings::new().expect("Failed to load settings");
    let euroleague_client = EuroleagueClient::new(settings.euroleague.base_url.clone());
    let client_data = web::Data::new(euroleague_client);

    println!("Starting server at http://{}:{}/", settings.server.host, settings.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(client_data.clone())
            .service(hello)
            .service(echo)
            .service(get_game)
    })
    .bind((settings.server.host.as_str(), settings.server.port))?
    .run()
    .await
}
