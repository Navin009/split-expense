use jsonwebtoken::{decode, errors::Result as JwtResult, Algorithm, DecodingKey, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use serde::{Deserialize, Serialize};

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

    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        // Retrieve the Authorization header
        if let Some(authorization) = req.headers().get_one("Authorization") {
            // The token should be in the format "Bearer <JWT>"
            if authorization.starts_with("Bearer ") {
                let token = &authorization[7..]; // Remove "Bearer " prefix
                match decode_jwt(token) {
                    Ok(claims) => Outcome::Success(JwtAuth(claims)),
                    Err(_) => Outcome::Error((
                        Status::Unauthorized,
                        std::io::Error::new(std::io::ErrorKind::Other, "Invalid JWT"),
                    )),
                }
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    std::io::Error::new(std::io::ErrorKind::Other, "Invalid token format"),
                ))
            }
        } else {
            Outcome::Error((
                Status::Unauthorized,
                std::io::Error::new(std::io::ErrorKind::Other, "Missing authorization header"),
            ))
        }
    }
}

// Function to decode JWT token
fn decode_jwt(token: &str) -> JwtResult<Claims> {
    let decoding_key = DecodingKey::from_secret(b"secret"); // Use a secret key for HMAC algorithm

    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(token, &decoding_key, &validation).map(|data| data.claims)
}
