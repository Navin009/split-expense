use core::str;

use base64::{prelude::BASE64_STANDARD, Engine};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
};

#[derive(Debug)]
pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = std::io::Error;

    async fn from_request(req: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(authorization) = req.headers().get_one("Authorization") {
            if authorization.starts_with("Basic ") {
                let encoded = &authorization[6..]; // Remove "Basic " prefix
                match BASE64_STANDARD.decode(encoded) {
                    Ok(decoded_bytes) => {
                        if let Ok(decoded_str) = str::from_utf8(&decoded_bytes) {
                            let parts: Vec<&str> = decoded_str.split(':').collect();
                            if parts.len() == 2 {
                                return Outcome::Success(BasicAuth {
                                    username: parts[0].to_string(),
                                    password: parts[1].to_string(),
                                });
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        }

        Outcome::Error((
            Status::Unauthorized,
            std::io::Error::new(std::io::ErrorKind::Other, "Invalid credentials"),
        ))
    }
}
