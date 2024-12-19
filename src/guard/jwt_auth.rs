use jsonwebtoken::{decode, errors::Result as JwtResult, Algorithm, DecodingKey, Validation};
use rocket::data::ByteUnit;
use rocket::http::{Header, Status};
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest};
use rocket::tokio::time::timeout;
use rocket::{tokio::time::Duration, Request};
use serde::{Deserialize, Serialize};
use std::time::Instant;

// Define the structure of your claims
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject of the token (usually user id)
    exp: usize,  // Expiration time (in seconds since epoch)
}

// Define a struct for the JWT guard
pub struct JwtAuth(pub Claims);

// Implement FromRequest for JwtAuth to create the guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtAuth {
    type Error = std::io::Error;

    async fn from_request(req: &'r rocket::Request<'r>) -> rocket::data::Outcome<'r, Self> {
        // Retrieve the Authorization header
        if let Some(authorization) = req.headers().get_one("Authorization") {
            // The token should be in the format "Bearer <JWT>"
            if authorization.starts_with("Bearer ") {
                let token = &authorization[7..]; // Remove "Bearer " prefix
                match decode_jwt(token) {
                    Ok(claims) => Outcome::Success(JwtAuth(claims)),
                    Err(_) => Outcome::Failure((
                        Status::Unauthorized,
                        std::io::Error::new(std::io::ErrorKind::Other, "Invalid JWT"),
                    )),
                }
            } else {
                Outcome::Failure((
                    Status::Unauthorized,
                    std::io::Error::new(std::io::ErrorKind::Other, "Invalid token format"),
                ))
            }
        } else {
            Outcome::Failure((
                Status::Unauthorized,
                std::io::Error::new(std::io::ErrorKind::Other, "Missing authorization header"),
            ))
        }
    }
}

// Function to decode JWT token
fn decode_jwt(token: &str) -> JwtResult<Claims> {
    let decoding_key = DecodingKey::from_secret(b"secret"); // Use a secret key for HMAC algorithm
    let validation = Validation {
        leeway: 0, // Adjust for any time variations if necessary
        validate_exp: true,
        algorithms: vec![Algorithm::HS256], // Algorithm used for signing the token
        ..Default::default()
    };
    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}
