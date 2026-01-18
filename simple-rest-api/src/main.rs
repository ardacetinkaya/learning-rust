mod euroleague;
mod extractors;
mod models;
mod settings;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use euroleague::euroleague::EuroleagueClient;
use extractors::{GamePathParams, ValidatedPath};
use models::greeting::Greeting;
use settings::Settings;

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
    params: ValidatedPath<GamePathParams>,
) -> impl Responder {
    match client.get_game(&params.0.season, params.0.code).await {
        Ok(game) => HttpResponse::Ok().json(game),
        Err(e) => {
            eprintln!("Error fetching game: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to fetch game data",
                "message": "An error occurred while retrieving game information"
            }))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let settings = Settings::new().expect("Failed to load settings");
    let euroleague_client = EuroleagueClient::new(settings.euroleague.base_url.clone());
    let client_data = web::Data::new(euroleague_client);

    println!(
        "Starting server at http://{}:{}/",
        settings.server.host, settings.server.port
    );

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
