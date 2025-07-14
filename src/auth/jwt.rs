use jsonwebtoken::{decode,encode,DecodingKey,EncodingKey,Header,Validation};
use serde::{Deserialize, Serialize};
use std::env;
use chrono::{Utc, Duration};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims{
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let exp = Utc::now() + Duration::hours(24);

    let claims = Claims {
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error>{
    let secret = env::var("JWT_SECRET").expect("KEY NOT FOUND !!");
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}