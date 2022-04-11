use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use crate::configuration::Settings;
use crate::routes;

pub struct Application {
    port: u16,
    server: Server
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{address}:{port}",
            address = config.application.host,
            port = config.application.port
        );

        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
        ).await?;

        Ok( Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn run(
    listener: TcpListener,
) -> Result<Server, anyhow::Error> {
    // let connection = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(routes::health_check::health_check))
            .route("/validate", web::post().to(routes::validation::validate_word))
            // .app_data(connection.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}

