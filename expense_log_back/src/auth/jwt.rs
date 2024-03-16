use app_properties::AppProperties;
use jsonwebtoken::{decode, encode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    exp: usize,
    iat: usize,
}

pub fn create_token() -> String {
    let app_props = AppProperties::new();
    let secret = app_props.get("jwt_secret");
    let now = chrono::Utc::now();
    let exp = now + chrono::Duration::hours(1);
    let iat = now;
    let claims = Claims {
        exp: exp.timestamp() as usize,
        iat: iat.timestamp() as usize,
    };
    let token = encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();
    token
}

pub fn verify_token(token: &str) -> bool {
    let app_props = AppProperties::new();
    let secret = app_props.get("jwt_secret");
    let key = DecodingKey::from_secret(secret.as_bytes());
    let token_data = decode::<Claims>(token, &key, &Validation::default());
    match token_data {
        Ok(_) => true,
        Err(_) => false,
    }
}
