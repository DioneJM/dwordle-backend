use reqwest::Client;
use uuid::Uuid;
use dwordle_backend::configuration::get_configuration;
use dwordle_backend::startup::Application;

pub struct TestApp {
    pub address: String,
    // pub connection: DbConnectionKind,
    pub port: u16,
    pub api_client: Client,
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut config = get_configuration()
            .expect("Failed to read config file");

        // config.database.database_name = Uuid::new_v4().to_string();
        config.application.port = 0;
        config
    };

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .unwrap();

    // // Create and migrate the database
    // configure_database(&configuration.database).await;

    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application");
    let application_port = application.port();
    let address = format!("http:127.0.0.1:{}", application_port);
    let _ = tokio::spawn(application.run_until_stopped());
    // We return the application address to the caller!
    let test_app = TestApp {
        address,
        port: application_port,
        // connection: get_database_connection(&configuration.database),
        api_client: client,
    };
    test_app
}
