use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use jsonwebtoken::{
    decode,
    DecodingKey,
    Validation,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            header.trim_start_matches("Bearer ")
        }
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    let jwt_secret =
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET missing");
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    ) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("JWT error: {:?}", err);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    req.extensions_mut()
        .insert(token_data.claims);

    Ok(next.run(req).await)
}