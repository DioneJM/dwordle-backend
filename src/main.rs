use crate::configuration::get_configuration;
use crate::startup::Application;

mod startup;
mod configuration;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let config = get_configuration()
        .expect("Failed to read config file");
    let application = Application::build(config.clone())
        .await
        .expect("Failed to build application");
    application.run_until_stopped().await?;
    Ok(())
}
