use actix_web::{dev::Payload, error, Error, FromRequest, HttpRequest};
use serde::de::DeserializeOwned;
use validator::Validate;
use std::future::{ready, Ready};
use std::fmt::Debug;

pub struct ValidatedPath<T>(pub T);

impl<T> FromRequest for ValidatedPath<T>
where
    T: DeserializeOwned + Validate + Debug,
{
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let path = req.match_info();
        
        // Try to deserialize the path parameters
        let params: T = match serde::de::Deserialize::deserialize(
            serde::de::value::MapDeserializer::<_, serde::de::value::Error>::new(
                path.iter().map(|(k, v)| (k.to_string(), v.to_string()))
            )
        ) {
            Ok(p) => p,
            Err(e) => {
                return ready(Err(error::ErrorBadRequest(
                    serde_json::json!({
                        "error": "Invalid path parameters",
                        "message": format!("Failed to parse path parameters: {}", e)
                    }).to_string()
                )));
            }
        };
        
        // Validate the deserialized parameters
        match params.validate() {
            Ok(_) => ready(Ok(ValidatedPath(params))),
            Err(e) => {
                ready(Err(error::ErrorBadRequest(
                    serde_json::json!({
                        "error": "Validation failed",
                        "message": format!("{}", e)
                    }).to_string()
                )))
            }
        }
    }
}


use serde::Deserialize;

#[derive(Debug, Deserialize, Validate)]
pub struct GamePathParams {
    pub season: String,
    
    #[validate(range(min = 1, max = 500, message = "Game code must be between 1 and 500"))]
    pub code: u32,
}
