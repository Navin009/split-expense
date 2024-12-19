use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};

pub struct LoggerFairing;

#[rocket::async_trait]
impl Fairing for LoggerFairing {
    fn info(&self) -> Info {
        Info {
            name: "Logger",
            kind: Kind::Ignite | Kind::Liftoff,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        log::info!("Incoming request: {} {}", request.method(), request.uri());
    }

    async fn on_response<'r>(&self, _: &'r Request<'_>, response: &mut Response<'r>) {
        log::info!("Response status: {}", response.status());
    }
}
