use reqwest::Client;
use crate::euroleague::models::Game;

pub struct EuroleagueClient {
    base_url: String,
    client: Client,
}

impl EuroleagueClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
        }
    }

    pub async fn get_game(&self, season_code: &str, game_code: u32) -> Result<Game, actix_web::Error> {
        let base = self.base_url.trim_end_matches('/');
        let url = format!("{}/v1/games?seasonCode={}&gameCode={}", base, season_code, game_code);
        
        println!("Fetching URL: {}", url);
        
        let response_text = self.client.get(&url).send().await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
            .text().await
            .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            
        // println!("Response text (len: {}): {}", response_text.len(), response_text);

        quick_xml::de::from_str(&response_text)
            .map_err(|e| {
                eprintln!("XML Parse Error: {:?}. Text: {}", e, response_text);
                actix_web::error::ErrorInternalServerError(e)
            })
    }
}