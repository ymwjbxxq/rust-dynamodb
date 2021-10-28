use serde_json::Value;
use crate::library::aws_client::AWSClient;
use lambda_runtime::{handler_fn, Error, Context};
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod dtos;
mod errors;
mod library;
mod models;
mod queries;
use library::lambda::handler::execute;

#[tokio::main]
async fn main() -> Result<(), Error> {
  // required to enable CloudWatch error logging by the runtime
  SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

  let config = aws_config::load_from_env().await;
  let aws_client = AWSClient::set_config(config);
  let client = aws_client.dynamo_client();

  lambda_runtime::run(handler_fn(|event: Value, ctx: Context| {
        execute(&client, event, ctx)
    })) 
    .await?;

  Ok(())
}
