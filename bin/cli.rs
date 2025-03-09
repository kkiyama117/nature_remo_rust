use dotenvy::dotenv;
use eyre::Result;
#[cfg(feature = "debug")]
use log::LevelFilter;
use nature_remo_api_client::NatureRemoAPIClient;

#[tokio::main]
#[async_backtrace::framed]
async fn main() -> Result<()> {
    #[cfg(feature = "debug")]
    {
        env_logger::Builder::from_default_env()
            .filter_module("nature_remo_api", LevelFilter::Debug)
            .init();
        color_eyre::install()?;
    }
    #[cfg(all(feature = "env_logger", not(feature = "debug")))]
    env_logger::init();
    dotenv()?;
    // Run demo
    let access_token = std::env::var("ACCESS_TOKEN")?;
    let mut client = NatureRemoAPIClient::new(&access_token);
    // let user = client.get_me().await?;
    // log::info!("{:#?}", user);
    log::info!("=-===================");
    let devices = client.get_devices().await?;
    let device = devices.iter().next().unwrap();
    log::info!("{:?}", devices);
    log::info!("{:?}", device);
    log::info!("==-==================");
    let appliances = client.get_appliances().await?;
    // log::info!("{:#?}", appliances);
    log::info!("{}", appliances);
    let app1 = appliances.iter().next().unwrap();
    let app1_id = &app1.id;
    log::info!("{:?}", app1_id);
    let signals = client.get_appliance_signals(app1_id.to_string()).await?;
    log::info!("{:?}", signals);
    log::info!("{:?}", client.last_rate_limit);
    Ok(())
}
